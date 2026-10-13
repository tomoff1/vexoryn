# VEXORYN 0.1.0 — validation du 13 octobre 2026

## Résultats observés

- Compilation TypeScript et Vite : réussie.
- Tests frontend : 6 réussis ; IPv4 privées, périmètre /24 et diagnostics cheats sans données inventées.
- Tests site : 4 réussis ; absence de téléchargement fictif, absence de release, filtrage des assets et des dépôts.
- Tests Rust : 9 réussis ; protection des destinations et chemins, parsers FTP, réponses bornées, listing/authentification FTP locale, aller-retour binaire de 128 Kio avec rejet d'une réponse de fin en échec, validation ELF et périmètre du scan.
- Compilation Windows x64 optimisée : réussie avec Rust MSVC, Tauri 2, WebView2.
- Installateur NSIS EXE : produit. Test d'installation silencieuse réussi, avec présence du programme, de la licence et des notices, et d'un raccourci bureau.
- Désinstallation NSIS de l'installation de test : réussie. Programme et raccourci retirés.
- Installateurs MSI FR et EN : produits. Installation MSI pas encore testée.
- Programme natif : démarrage et affichage du tableau de bord vérifiés. Capture réelle hors connexion incluse au site.
- Interface navigateur : navigation, affichage et diagnostic d'un conflit de versions saisi vérifiés.

Les installateurs finaux et leurs SHA-256 sont dans `artifacts/`. Les fichiers de compilation ont été placés sur D: pour éviter de saturer C:. Les dépendances exactes sont fixées par les deux lockfiles.

## Non validé

Aucune PS5 réelle n'a servi à ces tests. Le firmware 13.60 a été déclaré par l'utilisateur, pas détecté par le programme. Aucun payload n'a été exécuté, aucune installation PKG n'a été réalisée, aucun cheat n'a été activé. Les tests FTP locaux ne valident pas toutes les implémentations homebrew.

Les workflows GitHub préparés doivent encore être exécutés sur le dépôt distant. Une publication et un déploiement Pages ne sont attestés que par leurs résultats réels, indépendamment de la compilation locale.

La signature Authenticode, l'updater automatique signé, la reprise REST, l'annulation, les empreintes distantes, la télémétrie console, les adaptateurs PKG et cheats et la version native PS5 restent à développer.
