# dependency-visualizer

# Why

Répertorier tous types de dépendances en parcourant des fichiers de configurations.

# Quel type de dépendances

Ci dessous quelques exemples de dépendances, mais il faut garder à l'esprit que ces types sont libres.

Technique :

- Réseau
- CI
- Images docker
- Librairies

Métier

# Modélisation des dépendances

Sous formes d'un graph orienté.

## Les noeuds

Le noeud contient des informations qui lui sont propre par exemple :

- Pour une application, quel est le but de l'app ce quel fait etc ...
- Pour un noeud réseau quel est son nom de domaine, son ip etc...

lie yaml
regarde les workflow reutilisables
utilise l'edge pour récuper les infos dans le repo destination

```json
{
  "name": "library",
  "metadata": {
    "CI": {
      "workflows": {
        "build": {
          "edges": ["#/CI/workflows"],
          "file": "./.github/workflows/build-worklow.yaml"
        },
        "quality": {
          "edges": ["#/CI/workflows"],
          "file": "./.github/workflows/quality-worklow.yaml"
        }
      }
    },
    "functional": {
      "purpose": "Sell books to customers"
    }
  }
}
```

Un noeud contient des metadonnée classées par type dans l'exemple au dessus : CI et Functional.

## Les arcs

Les arcs représentent les connexions de dépendance entre les noeuds,
Le contexte actuel dépend toujours de la définition de l'edge...

```json
{
  "name": "library",
  "edges": {
    "CI": {
      "workflows": {
        "definition": "https://github/book-club/workflows/bootes.json",
        "enricher": "#/reusable_workflow"
      }
    },
    "functional": {
      "stock": {}
    }
  },
  "enrichers": {
    "reusable_workflow": {
      "environments": ["#/github"],
      "arguments": ["#/workflow"]
    }
  },
  "environments": {
    "github": {
      "TOKEN": "TEST"
    }
  },
  "arguments": {
    "workflow": ["-h"]
  }
}
```
