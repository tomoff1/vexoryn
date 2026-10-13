# Compatibilité et références communautaires

Recherche des sources primaires : 13 octobre 2026. Les références ci-dessous sont des projets amont, pas des composants inclus dans VEXORYN. Aucun payload tiers n'est redistribué et aucun firmware n'a été validé sur une console physique dans ce projet.

La compatibilité doit être évaluée pour **la version exacte du payload**, le firmware exact, le chargeur, le SDK et les services déjà lancés. Une borne annoncée par un projet tiers ne constitue pas une validation VEXORYN. Consultez les notes de sa release avant chaque envoi.

| Projet | Source primaire | Information vérifiée | Conséquence pour VEXORYN |
| --- | --- | --- | --- |
| etaHEN | [Dépôt etaHEN](https://github.com/etaHEN/etaHEN), [README](https://github.com/etaHEN/etaHEN/blob/main/README.md) | Le README décrit FTP, ps5debug et des options d'installation distinctes ; plusieurs services peuvent être désactivés. | Ne pas considérer ces services actifs simplement parce qu'etaHEN est mentionné. Certaines distributions sont BIN : un expéditeur ELF ne les accepte pas. |
| OnionHEN | [Dépôt amont](https://github.com/aydencharles/onionHEN), [releases](https://github.com/aydencharles/onionHEN/releases) | Environnement HEN et Toolbox ; les instructions de release utilisent un chargeur externe pour envoyer l'ELF. | Conserver les prérequis de la version choisie et éviter le lancement simultané de plusieurs HEN. |
| ps5debug | [Dépôt devcrono](https://github.com/devcrono/ps5debug) | Débogueur PS5 ; il existe plusieurs forks et variantes. | Un port ouvert ne prouve ni la version ni le bon fonctionnement du moteur cheats. Le protocole complet reste à intégrer. |
| ps5-payload-elfldr | [Dépôt amont](https://github.com/ps5-payload-dev/elfldr) | Chargeur ELF TCP utilisant habituellement 9021 ; son installation initiale dépend d'un chargeur déjà disponible. | L'envoi TCP correspond au transport documenté ; le résultat d'exécution doit être contrôlé séparément. |
| ps5-payload-ftpsrv | [Dépôt amont](https://github.com/ps5-payload-dev/ftpsrv) | Implémentation d'un service FTP pour PS5. | Configurer le port réel et vérifier les commandes prises en charge avant de promettre reprise ou intégrité distante. |
| ps5-payload-websrv | [Dépôt amont](https://github.com/ps5-payload-dev/websrv) | Serveur web, navigation de fichiers et lancement de homebrews ; le README documente notamment un listing JSON. | Piste d'adaptateur futur. L'explorateur actuel ne suppose pas cette API disponible. |
| ps5-payload-sdk | [SDK](https://github.com/ps5-payload-dev/sdk) | SDK communautaire pour payloads ELF ; exemples et compilation POSIX. | Base possible pour une phase native. Un SDK de payload ne fournit pas automatiquement un PKG installable et testé. |

## Firmware et identité de console

### Cas du firmware 13.60

Le firmware indiqué par l'utilisateur est 13.60 ; il n'a pas été lu sur une console par VEXORYN. Les [releases amont d'OnionHEN](https://github.com/aydencharles/onionHEN/releases) consultées annoncent 4.03–12.70, ce qui ne justifie pas leur envoi sur 13.60. [LegacyJB 1.2.1](https://github.com/Phoenixx1202/LegacyJB/releases) annonce des changements pour 13.60 et recommande explicitement une validation matérielle. Cette annonce concerne ce projet précis et ne prouve pas la compatibilité des autres HEN, serveurs ou outils de cheats.

Avant le premier essai VEXORYN, relever sur la console le service effectivement activé, sa version, son port et son chargeur. VEXORYN ne prépare pas la console et ne transforme pas la déclaration d'un firmware en compatibilité vérifiée.

La suite Windows ne requiert pas une version particulière pour dialoguer avec un serveur FTP ou un transport TCP compatible. Le payload exécuté, en revanche, peut être limité à des firmwares précis. Aucun intervalle global de compatibilité PS5 n'est annoncé par VEXORYN.

Le firmware, l'espace disque, la température, les processus, la liste de jeux et leurs versions nécessitent des commandes ou APIs connues. FTP et la connexion TCP seules ne suffisent pas. Tant qu'une API n'est pas intégrée et son retour vérifié, afficher « inconnu » ou « indisponible » est le comportement attendu.

## Cheats

Les extensions JSON, MC4 et SHN ne garantissent pas une compatibilité universelle. Elles dépendent du moteur, du jeu, de l'identifiant et de la version du patch. Les chemins changent entre moteurs et versions. Aucun répertoire universel ni fichier compatible ne doit être déduit de son extension seule. Une installation automatique doit attendre un adaptateur documenté, une sauvegarde et une détection de conflit.

## Étude d'un VEXORYN natif PS5

La voie réaliste est de sélectionner une révision du SDK communautaire, compiler d'abord son exemple `hello_world`, le charger sur une console autorisée puis ajouter une petite API en lecture seule. Le serveur web amont offre une piste d'interface locale dans le navigateur de la console. Une interface graphique native réclame ensuite la validation des bibliothèques, du runtime et des contrôles manette.

Avant un package installable : documenter les outils de packaging autorisés, contrôler les licences et l'absence d'artefacts propriétaires, compiler un vrai exécutable PS5, tester le lancement sur matériel puis générer et tester un package correspondant. Aucun SDK propriétaire Sony, faux PKG ou application PlayStation Store n'est inclus. La phase native est **une étude**, pas un livrable compilé.

Le SDK amont annonce GPLv3+ pour une partie importante de ses fichiers et des licences BSD pour certains headers : toute réutilisation future doit faire l'objet d'une analyse de licence avant distribution. Les liens ne sont pas une incorporation de ce code dans le client MIT.
