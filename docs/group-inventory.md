# Système de groupe + inventaire — rapport et syntaxe Rust

> Suite de [event-system.md](event-system.md). On réutilise **exactement** la
> même plomberie d'events (file mpsc par joueur + `broadcast_*` sur le
> `GameState`). Aucun nouveau format réseau : un event = une `Response`
> poussée spontanément.

---

## 1. Ce qui a été fait

### Découpage des fichiers (avant → après)

`dispatch.rs` était un gros `match` qui contenait toute la logique. Il est
devenu un **routeur** : il choisit le bon handler, point. Toute la logique
métier est rangée par domaine dans `network/handlers/`.

```
server/src/
├── protocol/
│   └── command.rs        # + ChatScope, GroupAction, TAKE/DROP/INVENTORY/GROUP/CHAT
├── state/
│   ├── game.rs           # players + groups + world + helpers d'envoi/groupe
│   ├── player.rs         # + inventory, + group
│   ├── group.rs          # NOUVEAU : Group, GroupId
│   └── world.rs          # NOUVEAU : WorldState (objets au sol, dynamique)
└── network/
    ├── dispatch.rs       # routeur fin (1 bras = 1 handler)
    └── handlers/         # NOUVEAU dossier
        ├── mod.rs        # resolve_item() (id ou nom d'affichage)
        ├── session.rs    # CONNECT, WHO, disconnect()
        ├── chat.rs       # CHAT GLOBAL/ROOM/GROUP
        ├── world.rs      # LOOK (vraie salle : items/npcs/joueurs/sorties)
        ├── inventory.rs  # TAKE, DROP, INVENTORY
        └── group.rs      # GROUP CREATE/INVITE/JOIN/LEAVE
```

Règle d'or de cette archi : **`config` = monde statique en lecture seule**
(la carte, chargée de `world.yml`), **`state` = ce qui change en jeu**
(qui est où, qui porte quoi, quels groupes existent).

### Commandes ajoutées (conformes RFC 42TAP)

| Commande | Effet | Event(s) émis |
|---|---|---|
| `INVENTORY` | liste tes objets | — |
| `TAKE <objet>` | ramasse au sol | `item_taken` à la salle |
| `DROP <objet>` | repose au sol | `item_dropped` à la salle |
| `LOOK` | état réel de la salle | — |
| `CHAT GLOBAL\|ROOM\|GROUP <msg>` | chat selon portée | `chat_*` aux destinataires |
| `GROUP CREATE` | crée un groupe | — |
| `GROUP INVITE <joueur>` | invite | `group_invite` à l'invité |
| `GROUP JOIN <leader>` | rejoint | `group_join` au groupe |
| `GROUP LEAVE` | quitte / dissout | `group_leave` ou `group_disband` |
| déconnexion | nettoyage | `group_leave/disband` + `presence_leave` |

`<objet>` accepte l'**id** (`herbs`) **ou le nom d'affichage** (`Healing
Herbs`), insensible à la casse, multi-mots — exigence du sujet (Dynamic Item
Management).

### Choix de design (à mettre dans le README, le sujet l'exige)

- **Objet ramassable** : un `TAKE` ne marche que si l'objet est au sol *et*
  `obtainable: true` dans `world.yml` (sinon c'est du décor → `ITEM_NOT_FOUND`).
- **Groupe par invitation** : `GROUP JOIN` n'est accepté que si le joueur a
  d'abord reçu une invitation (`GROUP INVITE`).
- **Le leader quitte → groupe dissous** (`group_disband`), choix simple et
  prévisible plutôt qu'un transfert de leadership.
- **Aucune persistance** (conforme : « state may reset on restart »). Les
  objets au sol sont ré-initialisés depuis `world.yml` au démarrage.

---

## 2. Vérification par rapport au sujet

✅ Conforme :
- Toutes les commandes Groupe/Inventaire de la RFC sont implémentées + events.
- `TAKE` retire l'objet de la salle (pas de duplication), `DROP` le rend
  disponible aux autres → testé à deux clients.
- Noms d'objets multi-mots et par id/nom : OK.
- Déconnexion : on **retire l'état avant** de diffuser le départ (règle
  globale du sujet).
- Logs structurés JSON avec niveaux + timestamps (déjà en place via
  `tracing`), mes handlers émettent des champs structurés
  (`player`, `item`, `group`).

⚠️ **Déviations à documenter dans le README** (le sujet : « Any deviation
from the protocol must be clearly documented »). Ce ne sont pas des bugs,
c'est un choix d'équipe assumé, mais il faut l'écrire :

1. **Format réseau JSON** au lieu du texte RFC (`OK taken=item.herbs`).
   Vous renvoyez `{"status":"ok","type":"take","data":{"taken":"herbs"}}`.
   Or le sujet impose que les clients CLI/GUI soient **interchangeables
   entre groupes** → un client d'un autre groupe parlant le vrai RFC ne
   comprendra pas ce JSON. C'est le rôle du crate `bridge/` (encore vide) :
   il devra traduire JSON ⇄ texte RFC. **À mentionner explicitement.**
2. **Codes d'erreur** : `CONNECT` renvoie `409` au lieu de `201 NAME_IN_USE`
   (RFC). Mes nouvelles commandes utilisent les bons codes RFC (401, 402,
   404). À uniformiser ou à documenter.
3. `MOVE` n'est pas implémenté → tout le monde reste dans `start`. Hors
   périmètre de cette tâche, mais à faire pour la map en boucle exigée.

---

## 3. La syntaxe Rust à connaître (pour défendre en peer-eval)

> Tu dois pouvoir expliquer **chaque** point ci-dessous. Ils reviennent
> partout dans le code ajouté.

### 3.1 `enum` avec données + `match`

```rust
pub enum GroupAction {
    Create,                       // variante sans donnée
    Invite { target: String },    // variante "struct-like"
    Join   { leader: String },
    Leave,
}
```

Un `enum` Rust peut **porter des données différentes par variante** (≠ enum
C). On le lit avec `match`, qui est **exhaustif** : le compilateur refuse de
compiler si tu oublies une variante.

```rust
match action {
    GroupAction::Create => { /* ... */ }
    GroupAction::Invite { target } => { /* `target` est extrait ici */ }
    GroupAction::Join { leader } => { /* ... */ }
    GroupAction::Leave => { /* ... */ }
}
```

### 3.2 `Option<T>` et `Result<T, E>`

Rust n'a **pas de `null`** ni d'exceptions. Deux types pour ça :

- `Option<T>` = `Some(valeur)` ou `None` (« il y a peut-être une valeur »).
  Ex : `player.group: Option<GroupId>` (le joueur est *peut-être* en groupe).
- `Result<T, E>` = `Ok(valeur)` ou `Err(erreur)`. Ex : `Command::parse`
  renvoie `Result<Command, Response>` : soit une commande, soit l'erreur à
  renvoyer au client.

Le motif qu'on utilise sans arrêt — « extraire ou sortir tôt » :

```rust
let name = match state.name_of(addr) {
    Some(n) => n,                                  // on a le nom -> on continue
    None => return Response::error(403, "Connect first"), // sinon on quitte
};
```

`?` est le raccourci de ce motif pour `Result` (propage l'`Err`) :

```rust
target: require(arg, "GROUP INVITE requires a username")?,
//                                                       ^ si Err -> return Err
```

### 3.3 Propriété, emprunts, `&` / `&mut`

C'est LE point central de Rust. Une valeur a **un seul propriétaire**. On
peut prêter une référence :

- `&T` : prêt **partagé**, lecture seule, autant qu'on veut en même temps.
- `&mut T` : prêt **exclusif**, lecture/écriture, **un seul à la fois**.

Conséquence concrète dans `take()` : on ne peut pas lire `state.world` et le
modifier dans la même expression. D'où ce découpage volontaire :

```rust
// 1) on LIT (emprunt partagé), puis l'emprunt se termine (on a un bool)
let present = state.world.items_in(&room).iter().any(|i| i == &item_id);
if !present { return Response::error(404, "ITEM_NOT_FOUND"); }

// 2) maintenant on peut MODIFIER (emprunt exclusif)
state.world.remove_item(&room, &item_id);
```

`.clone()` sert à **sortir du problème d'emprunt** : on copie la donnée pour
en devenir propriétaire et ne plus dépendre de l'emprunt (ex :
`p.name.clone()`, `members.clone()`). À utiliser quand c'est plus simple que
de jongler avec les durées de vie — ici les volumes sont petits.

### 3.4 `String` vs `&str`

- `&str` : vue empruntée sur du texte (ex : `addr: &str`, un littéral
  `"start"`).
- `String` : texte **possédé**, modifiable, vit sur le tas.

`"start".to_string()` ou `.to_owned()` = passer de l'emprunt au possédé.
`&ma_string` = repasser en `&str`. On choisit `String` quand on doit stocker
la donnée (dans une struct, une `HashMap`…), `&str` pour juste la lire.

### 3.5 `HashMap`, `Vec`, `HashSet`

```rust
players: HashMap<String, Player>   // clé = nom -> joueur
inventory: Vec<String>             // liste ordonnée d'ids
invited: HashSet<String>           // ensemble (pas de doublon, test rapide)
```

- `map.get(&k)` → `Option<&V>` ; `map.get_mut(&k)` → `Option<&mut V>`.
- `map[&k]` → accès direct **mais panique si absent** (je l'utilise seulement
  juste après avoir vérifié la présence).
- `vec.iter().position(|x| ...)` → `Option<usize>` (l'index), puis
  `vec.remove(idx)`.
- `.entry(k).or_default()` → « récupère, ou crée la valeur par défaut ».

### 3.6 Closures et itérateurs

```rust
state.players.values().find(|p| p.addr == addr).map(|p| p.name.clone())
```

- `|p| p.addr == addr` : une **closure** (fonction anonyme) qui capture
  `addr`.
- `.values()` / `.iter()` : un **itérateur** paresseux.
- `.find(...)` → `Option<&T>` (premier qui matche).
- `.map(...)` : transforme l'intérieur d'un `Option` **seulement s'il existe**.
- `.filter(...).map(...).collect()` : pipeline → on construit un `Vec`.

`.iter().any(|i| i == &item_id)` = « est-ce qu'au moins un élément vérifie… ».

### 3.7 `async` / `.await` et le partage entre tâches

Chaque connexion tourne dans une tâche `tokio`. L'état est partagé via :

```rust
Arc<RwLock<GameState>>
```

- `Arc<T>` : pointeur **compté** → plusieurs tâches possèdent le même état.
- `RwLock<T>` : verrou « plusieurs lecteurs **ou** un seul écrivain ».
  - `state.read().await` → lecture (plusieurs handlers en parallèle).
  - `state.write().await` → écriture (exclusif : `TAKE`, `GROUP`…).
- `.await` : « rends la main tant que ce n'est pas prêt » (ici : tant que le
  verrou n'est pas disponible). Ça n'est pas bloquant pour les autres tâches.

Important : `tx.send(...)` (pousser un event) est **non bloquant**, donc on a
le droit de le faire en tenant le verrou — c'est ce qui permet
`broadcast_room` pendant qu'on tient `write()`.

### 3.8 `impl`, méthodes, `self`

```rust
impl WorldState {
    pub fn from_config() -> Self { ... }      // pas de self -> "constructeur"
    pub fn items_in(&self, room: &str) -> &[String] { ... }   // lecture
    pub fn remove_item(&mut self, ...) -> bool { ... }        // mutation
}
```

`&self` = la méthode lit l'objet ; `&mut self` = elle le modifie ; pas de
`self` = fonction associée (appelée `WorldState::from_config()`).

### 3.9 Le `;` final et les expressions

En Rust presque tout est une **expression** qui renvoie une valeur. Une
fonction renvoie sa **dernière expression sans `;`** :

```rust
Response::ok("drop", json!({ "dropped": item_id }))   // <- pas de ; = valeur renvoyée
```

Un `match` ou un `if` renvoie aussi une valeur, d'où le style
`let scope = match ... { ... };`.

---

## 4. Recette : ajouter une commande plus tard

1. Une variante dans `enum Command` (+ parsing dans `command.rs`).
2. Un `async fn` dans le bon fichier de `network/handlers/`.
3. Un bras dans `dispatch.rs` qui appelle ce handler.
4. Pour notifier quelqu'un : `state.send_to / broadcast_room /
   broadcast_group / broadcast_all` avec une `Response::ok("event", …)`.

Tu ne touches **jamais** à la plomberie (file mpsc, tâche d'écriture).

---

## 5. Comment tester (multijoueur)

Deux terminaux `nc 127.0.0.1 4000` côte à côte :

```
# terminal A          # terminal B
CONNECT alice          CONNECT bob          (A reçoit presence_enter bob)
LOOK                                        (items: herbs, lantern)
TAKE Healing Herbs                          (B reçoit item_taken)
INVENTORY                                   -> ["herbs"]
GROUP CREATE
GROUP INVITE bob       (B reçoit group_invite)
                       GROUP JOIN alice     (A reçoit group_join)
CHAT GROUP salut       (B reçoit chat_group)
                       GROUP LEAVE          (A reçoit group_leave)
```
