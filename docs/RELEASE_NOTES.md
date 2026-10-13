# VEXORYN 0.1.0 — première version technique Windows

Application Windows 10/11 x64 en Tauri 2, Rust, React et TypeScript. Interface FR/EN, trois thèmes sombres et identité VEXORYN.

## Fonctions livrées

- Connexion FTP manuelle aux adresses IPv4 privées.
- Explorateur FTP passif, création de dossiers, suppressions confirmées, envoi et téléchargement réels.
- File de transferts, progression, vitesse, SHA-256 local, réponse finale FTP vérifiée et contrôle de taille quand le serveur le permet.
- Sauvegarde des fichiers avant remplacement.
- Tests TCP et découverte autorisée d'un seul sous-réseau privé /24 ; les résultats sont des candidats, sans identité PS5 certifiée.
- Envoi d'ELF x86-64 à un chargeur TCP compatible, après confirmation explicite ; exécution jamais déclarée confirmée.
- Smart Cheat Diagnostics à partir des observations saisies, historique local et consultation des releases GitHub.
- Installateur EXE, MSI FR et EN, raccourci bureau, désinstalleur et notices de licences.

## Validation

19 tests logiciels réussis, compilation Windows locale réussie, démarrage natif vérifié, installation et désinstallation NSIS testées. Les MSI sont compilés mais leur installation n'a pas été testée. Les SHA-256 des fichiers distribués sont dans SHA256SUMS.txt.

**Aucune validation sur PS5 réelle.** Le firmware 13.60 communiqué n'a pas été détecté sur matériel et n'est pas déclaré compatible avec tous les payloads. Un port ouvert ne prouve pas un jailbreak. Un ELF transmis ne prouve pas son exécution. Un PKG transféré n'est pas installé.

## Travail restant

Adaptateurs d'installation PKG avec confirmation console, détection fiable des jeux et versions, cheats par moteur, télémétrie, reprise REST, annulation, SHA-256 distant, sauvegardes et restaurations structurées, plugins validés, updater signé et homebrew natif PS5. Les installateurs ne sont pas signés par Authenticode.

Utilisez uniquement vos consoles et des fichiers personnels ou autorisés sur un réseau privé de confiance. Aucun exploit, payload tiers, jeu, clé ou cheat tiers n'est distribué.
