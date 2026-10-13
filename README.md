# VEXORYN — PS5 Console Suite

Application Windows indépendante en Rust / Tauri 2 / React / TypeScript pour communiquer avec les services homebrew d'une console sur un réseau privé. Interface française et anglaise, thèmes sombre violet, bleu et rouge. Identité et métadonnées : VEXORYN uniquement.

[Site VEXORYN](https://tomoff1.github.io/vexoryn/) · [Téléchargements Windows et SHA-256](https://github.com/tomoff1/vexoryn/releases/tag/v0.1.0) · [Notes de version](docs/RELEASE_NOTES.md) · [Rapport de validation](docs/VALIDATION_REPORT.md)

Cette première version contient du code réseau réel. Elle doit encore être validée sur une PS5 matérielle : des tests locaux ne prouvent pas la compatibilité de chaque serveur homebrew. Un port ouvert n'identifie pas une PS5. Une transmission ELF n'est pas une confirmation d'exécution. Un transfert PKG n'est pas une installation.

## Démarrer

Suivez [le guide débutant](docs/BUILD.md). Pour une machine Windows avec Node.js, Rust MSVC, les outils C++ et WebView2 déjà installés :

```powershell
npm ci
npm run tauri -- dev
```

## Vérifier et compiler

```powershell
npm test
npm run build
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri -- build --bundles nsis,msi
```

Les installateurs réellement produits se trouvent dans `src-tauri/target/release/bundle/nsis/` et `src-tauri/target/release/bundle/msi/`. Leur présence n'est annoncée qu'après une compilation réussie. Le fichier `.exe` dans `target/release` est le programme, pas l'installateur.

## Guides

- [Utilisation et test avec votre console](docs/USER_GUIDE.md)
- [Publication GitHub Releases et Pages](docs/PUBLISH.md)
- [Compatibilité et sources primaires](docs/COMPATIBILITY.md)
- [État des fonctionnalités et feuille de route](docs/STATUS.md)
- [Plan de validation matérielle](docs/PS5_VALIDATION.md)
- [Licences et attributions](THIRD_PARTY.md)

Le site statique est dans `website/`. Il n'annonce aucune release inexistante. Le workflow Pages renseigne automatiquement le véritable dépôt ; localement, `website/config.json` utilise la destination choisie `tomoff1/vexoryn`. Le dépôt local est initialisé avec cette adresse distante ; aucune publication n'est présumée.

## Architecture

`src/` contient l'interface React. `src-tauri/` contient les commandes Rust, transports réseau et configuration des installateurs. `website/` est indépendant du client et fonctionne sur GitHub Pages. `.github/workflows/` vérifie les sources, compile les bundles Windows, calcule leurs SHA-256 et prépare une release brouillon.

## Confidentialité et limites

Aucune télémétrie n'est prévue. Ne publiez pas d'adresses IP, de chemins personnels ou de journaux bruts dans les Issues. FTP transmet ses données sans chiffrement ; utilisez votre réseau privé de confiance. VEXORYN ne distribue ni exploit, jeu, clé propriétaire, payload tiers ou cheat tiers. Seuls les fichiers personnels ou autorisés doivent être utilisés. Aucune opération réseau n'est exposée sur Internet par défaut.

Licence du code VEXORYN : MIT, voir [LICENSE](LICENSE). Les marques citées appartiennent à leurs détenteurs. Projet indépendant de Sony Interactive Entertainment.
