# Compiler VEXORYN sur Windows

## Sur le PC utilisé pour créer ce projet

Rust a été installé de façon portable dans `D:\VEXORYN-build` parce que C: était presque plein. Depuis le dossier du projet, la commande suivante sélectionne automatiquement cette installation, exécute les tests et construit les installateurs :

```powershell
.\scripts\build-local.ps1
```

Pour lancer l'application en développement : `.\scripts\build-local.ps1 -Dev`. Pour exécuter les vérifications seules : `.\scripts\build-local.ps1 -TestsOnly`. Les fichiers de compilation utilisent `D:\VEXORYN-build\target`. Le dossier `artifacts` contient les livrables copiés après la compilation initiale quand elle réussit. Sur un autre PC, suivez les prérequis ci-dessous.

## Installer les prérequis une fois

1. Installez [Node.js 22 LTS](https://nodejs.org/en/download), avec npm.
2. Installez [Rust via rustup](https://rustup.rs/). Conservez le profil stable et la cible Windows MSVC x64.
3. Installez [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) avec la charge de travail **Développement Desktop en C++**, les outils MSVC et un SDK Windows 10 ou 11.
4. Vérifiez que [Microsoft Edge WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) est installé. Windows 11 le fournit généralement.
5. Fermez puis rouvrez PowerShell après ces installations.

La liste officielle et à jour des prérequis est dans [Tauri 2](https://v2.tauri.app/start/prerequisites/).

## Ouvrir le projet

Dans l'Explorateur Windows, ouvrez le dossier VEXORYN. Cliquez la barre d'adresse, saisissez `powershell`, puis Entrée. Vérifiez :

```powershell
node --version
npm --version
rustc --version
cargo --version
```

Puis :

```powershell
npm ci
npm run tauri -- dev
```

`npm run dev` ouvre uniquement l'interface navigateur : les commandes réseau Rust nécessitent la fenêtre Tauri. Les erreurs doivent rester affichées lorsqu'aucun backend n'est disponible.

## Tester puis créer les installateurs

```powershell
npm test
npm run build
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri -- build --bundles nsis,msi
```

La première compilation télécharge les dépendances et outils de packaging. Prévoyez une connexion Internet et plusieurs gigaoctets libres. Ouvrez `src-tauri/target/release/bundle/nsis` pour le `.exe` d'installation, `src-tauri/target/release/bundle/msi` pour le `.msi`.

Si MSI échoue avec `light.exe`, vérifiez la fonctionnalité optionnelle VBScript conformément au [guide Tauri Windows](https://v2.tauri.app/distribute/windows-installer/). Il reste possible de compiler seulement NSIS : `npm run tauri -- build --bundles nsis`.

Un installateur sans certificat de signature Windows peut déclencher SmartScreen. La signature Authenticode nécessite un certificat et n'est pas configurée dans cette version.

## Vérifier une somme de contrôle

```powershell
Get-FileHash -Algorithm SHA256 -LiteralPath '.\VEXORYN-installer.exe'
```

Remplacez ce nom d'exemple par celui du fichier réellement téléchargé. Comparez le résultat à `SHA256SUMS.txt` de la même release.
