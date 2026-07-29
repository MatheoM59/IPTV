# Référence — réponse de `player_api.php`

Relevé du 2026-07-29 sur `http://pl-ott.com/player_api.php`.

**Rappel :** un `User-Agent` de lecteur est obligatoire (`VLC/3.0.20 LibVLC/3.0.20`),
sinon le panel répond `401` dès qu'on lui présente des identifiants.

⚠️ Les identifiants sont masqués ici. Le panel les renvoie en clair dans sa réponse —
ne jamais logger ni afficher `user_info.password`.

## Réponse brute (remise en forme)

```json
{
  "user_info": {
    "username": "«masqué»",
    "password": "«masqué»",
    "message": "",
    "auth": 1,
    "status": "Active",
    "exp_date": "1811788107",
    "is_trial": "0",
    "active_cons": "0",
    "created_at": "1780252107",
    "max_connections": "2",
    "allowed_output_formats": ["ts"]
  },
  "server_info": {
    "url": "pl-ott.com",
    "port": "80",
    "https_port": "25463",
    "server_protocol": "http",
    "rtmp_port": "25462",
    "timezone": "Europe/Paris",
    "timestamp_now": 1785331815,
    "time_now": "2026-07-29 15:30:15",
    "process": true
  }
}
```

## `user_info`

| Champ | Valeur | Type JSON | Signification |
|---|---|---|---|
| `username` | `"«masqué»"` | chaîne | Identifiant du compte |
| `password` | `"«masqué»"` | chaîne | Renvoyé en clair — ne jamais logger |
| `message` | `""` | chaîne | Message du fournisseur, vide ici |
| `auth` | `1` | **nombre** | `1` = authentifié, `0` = refusé |
| `status` | `"Active"` | chaîne | État de l'abonnement |
| `exp_date` | `"1811788107"` | **chaîne** | Timestamp Unix → 2027-05-31 |
| `is_trial` | `"0"` | **chaîne** | `"0"` = compte normal |
| `active_cons` | `"0"` | **chaîne** | Connexions actuellement ouvertes |
| `created_at` | `"1780252107"` | **chaîne** | Timestamp Unix → 2026-05-31 |
| `max_connections` | `"2"` | **chaîne** | 2 flux simultanés maximum |
| `allowed_output_formats` | `["ts"]` | tableau de chaînes | **Uniquement du MPEG-TS, pas de HLS** |

## `server_info`

| Champ | Valeur | Type JSON | Signification |
|---|---|---|---|
| `url` | `"pl-ott.com"` | chaîne | Hôte du panel |
| `port` | `"80"` | **chaîne** | Port HTTP |
| `https_port` | `"25463"` | **chaîne** | Port HTTPS — **certificat auto-signé** |
| `server_protocol` | `"http"` | chaîne | Protocole par défaut |
| `rtmp_port` | `"25462"` | **chaîne** | Port RTMP |
| `timezone` | `"Europe/Paris"` | chaîne | Fuseau du serveur |
| `timestamp_now` | `1785331815` | **nombre** | Heure serveur, timestamp Unix |
| `time_now` | `"2026-07-29 15:30:15"` | chaîne | Même heure, lisible |
| `process` | `true` | booléen | Indicateur interne du panel |

## ⚠️ Le piège : les types sont incohérents

Cette API mélange les représentations **sans aucune logique** :

- `auth` est un **nombre** (`1`) mais `is_trial` est une **chaîne** (`"0"`)
- `timestamp_now` est un **nombre** mais `exp_date` et `created_at` sont des **chaînes**
- `max_connections` est une **chaîne** (`"2"`) alors que c'est manifestement un entier

En JavaScript on ne s'en apercevrait presque pas. En Rust, chaque champ doit être déclaré
avec le bon type, sinon la désérialisation échoue à l'exécution.

**Règle simple :** dans la colonne « Type JSON » ci-dessus, une valeur entre guillemets
est une chaîne — quoi qu'elle contienne.

---

# `action=get_live_categories` — catégories de chaînes

Relevé du 2026-07-29. Même endpoint, avec `&action=get_live_categories` en plus.

## Réponse brute (extrait remis en forme)

```json
[
  { "category_id": "3",   "category_name": "|FR| FRANCE FHD",     "parent_id": 0 },
  { "category_id": "5",   "category_name": "|FR| FRANCE HD",      "parent_id": 0 },
  { "category_id": "170", "category_name": "|FR| FRANCE SD",      "parent_id": 0 },
  { "category_id": "4",   "category_name": "|FR| FRANCE HEVC",    "parent_id": 0 },
  { "category_id": "298", "category_name": "|FR| FRANCE ENFANTS", "parent_id": 0 },
  { "category_id": "249", "category_name": "|FR| FRANCE MUSIQUE", "parent_id": 0 },
  { "category_id": "7",   "category_name": "|FR| FRANCE SPORT",   "parent_id": 0 }
]
```

La liste continue bien au-delà de ces sept entrées.

## 🔑 La différence de structure

**La racine est un tableau, pas un objet.**

Pour l'authentification, il fallait une struct enveloppe (`ApiResponse`) parce que les
données étaient rangées sous une clé `user_info`. Ici il n'y a aucune clé au-dessus :
le document *est* la liste.

Conséquence : **pas de struct enveloppe**. Une seule struct décrit *un* élément, et le
type de désérialisation est une liste de cette struct.

Tous les éléments ont rigoureusement la même forme — c'est ce qui permet de n'écrire
qu'une seule struct pour les milliers d'entrées.

## Champs d'un élément

| Champ | Exemple | Type JSON | Signification |
|---|---|---|---|
| `category_id` | `"3"` | **chaîne** | Identifiant, à réutiliser pour demander les chaînes de cette catégorie |
| `category_name` | `"\|FR\| FRANCE FHD"` | chaîne | Libellé affichable |
| `parent_id` | `0` | **nombre** | Catégorie parente. Vaut `0` partout ici — hiérarchie non utilisée par ce panel |

⚠️ Même piège que pour l'authentification : `category_id` est une **chaîne** malgré son
apparence numérique, alors que `parent_id` est un **nombre**. Fie-toi aux guillemets,
jamais à ce que la valeur semble représenter.

## Remarques utiles pour l'app

- Les libellés portent un préfixe de langue entre barres verticales (`|FR|`). Utile plus
  tard pour regrouper ou filtrer par pays, mais à ne pas parser naïvement : rien ne
  garantit que tous les fournisseurs suivent cette convention.
- La catégorie **`|FR| FRANCE HEVC`** confirme la présence de chaînes en H.265, que les
  navigateurs ne décodent pas de façon fiable. Argument concret pour le lecteur natif.
- `parent_id` vaut `0` sur toutes les entrées observées : inutile de construire un arbre
  de catégories pour ce panel, une liste plate suffit.

---

## Autres endpoints à explorer

Tous prennent `?username=…&password=…` et un paramètre `action` :

Trois catalogues **indépendants**, chacun avec ses catégories et son contenu :

| `action` | Contenu |
|---|---|
| *(aucun)* | Infos compte + serveur |
| `get_live_categories` | Catégories de chaînes TV |
| `get_live_streams` | Chaînes TV, avec `stream_id` et `stream_icon` |
| `get_vod_categories` | Catégories de films |
| `get_vod_streams` | Films |
| `get_series_categories` | Catégories de séries |
| `get_series` | Séries |
| `get_series_info&series_id=…` | Saisons et épisodes d'une série |
| `get_short_epg&stream_id=…` | Programme en cours d'une chaîne |

Chaque `action` de catégories accepte aussi `&category_id=…` sur l'appel de contenu
correspondant, pour ne demander qu'une catégorie au lieu du catalogue entier.

**Les trois endpoints de catégories renvoient la même forme** (`category_id`,
`category_name`, `parent_id`) — une seule struct `Category` suffit pour les trois.
En revanche les contenus diffèrent : un film a une `container_extension` et une note,
une série a un `series_id` au lieu d'un `stream_id`. Il faudra une struct par catalogue.

L'EPG complet est ailleurs : `xmltv.php?username=…&password=…` (format XMLTV, pas JSON).