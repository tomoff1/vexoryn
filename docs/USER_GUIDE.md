# Utiliser VEXORYN

## Première connexion

Votre console doit déjà disposer d'un environnement homebrew compatible. VEXORYN ne réalise pas son jailbreak. Connectez la console et le PC au même réseau privé. Activez le service FTP de votre environnement ; relevez son adresse IP et son port dans la console. Ne supposez pas un port standard si votre environnement en expose un autre.

Dans VEXORYN, renseignez le nom d'affichage, l'adresse IP locale et les ports des services. Lancez le diagnostic avant d'ouvrir l'explorateur. Vérifiez les identifiants FTP si votre service en exige. Le nom est choisi par vous ; l'application ne déduit pas l'identité PS5 d'un simple port ouvert.

## Fichiers

Parcourez un dossier distant, puis choisissez un fichier local pour l'envoyer ou un fichier distant pour le télécharger. Vérifiez la destination. Les suppressions demandent une confirmation. Un remplacement distant conserve d'abord le fichier original sous un nom `.vexoryn-backup-TIMESTAMP` ; si cette sauvegarde échoue, le remplacement est refusé. Les téléchargements utilisent un fichier partiel et préservent également une destination existante avant finalisation. Pour un envoi interrompu, contrôlez la taille du fichier distant ; n'interprétez pas un fichier partiel comme un transfert terminé.

FTP n'assure pas à lui seul une empreinte SHA-256 distante. Pour une vérification forte, téléchargez le fichier transféré et comparez ses empreintes locales. Un contrôle de taille est moins fort qu'un contrôle cryptographique. La reprise automatique REST et l'annulation ne sont pas implémentées dans cette version. MLSD est utilisé en priorité, puis le format UNIX LIST ; les listings incompatibles et liens symboliques ne sont pas pris en charge.

## ELF et payloads

Ouvrez les sources officielles liées au catalogue, choisissez une version compatible avec votre console puis sélectionnez votre propre fichier ELF. L'accessibilité du chargeur est nécessaire ; un port TCP ouvert ne garantit pas son protocole. Un statut « envoyé » signifie que le transport a terminé, pas que le payload est opérationnel. Vérifiez le résultat sur la console.

N'exécutez pas simultanément etaHEN et OnionHEN. Un outil peut intégrer FTP ou ps5debug : évitez un second serveur sur le même port. Sans API de processus fiable, VEXORYN ne peut pas détecter tous ces conflits.

## Cheats et diagnostics

Utilisez uniquement des cheats pour vos parties solo. Fournissez l'identifiant CUSA/PPSA et la version réellement lue dans la console ou dans une source fiable. Le message « Unable to detect patch version » peut provenir d'un patch non reconnu, d'un jeu non lancé ou d'un moteur ne sachant pas lire ses métadonnées. Ce sont des hypothèses de diagnostic, pas une détection automatique ni une garantie de compatibilité.

## Mises à jour et désinstallation

Configurez le véritable dépôt de publication pour consulter ses releases. La vérification indique une version disponible ; l'installation automatique signée est une étape ultérieure. Téléchargez uniquement une release publiée avec ses sommes SHA-256.

Pour désinstaller : **Paramètres Windows → Applications → VEXORYN → Désinstaller**. Les fichiers console ne sont pas supprimés par la désinstallation Windows. L'historique se trouve dans `%LOCALAPPDATA%\app.vexoryn.console\history.json` ; conservez-en une copie avant de retirer les données locales si vous en avez besoin. Il n'existe pas encore de bouton d'export automatique.
