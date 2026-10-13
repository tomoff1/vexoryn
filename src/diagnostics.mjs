/** Validate canonical local IPv4 addresses; no hostnames or public destinations. */
export function isPrivateIPv4(ip) {
  if (typeof ip !== 'string' || !/^(0|[1-9]\d{0,2})(\.(0|[1-9]\d{0,2})){3}$/.test(ip)) return false;
  const octets = ip.split('.').map(Number);
  if (octets.some(value => value > 255)) return false;
  return octets[0] === 10 || (octets[0] === 192 && octets[1] === 168) || (octets[0] === 172 && octets[1] >= 16 && octets[1] <= 31);
}

/** Discovery is restricted to a single RFC1918 /24, never loopback or Internet. */
export function isPrivateSubnet24(subnet) {
  if (typeof subnet !== 'string' || !subnet.endsWith('.0/24')) return false;
  const address = subnet.slice(0, -3);
  return isPrivateIPv4(address) && !address.startsWith('127.');
}

export function derivePrivateSubnet(ip) {
  if (!isPrivateIPv4(ip) || ip.startsWith('127.')) return '';
  return `${ip.split('.').slice(0, 3).join('.')}.0/24`;
}

/** These are hypotheses from user-supplied facts, never detected console telemetry. */
export function analyzeCheatFacts({ gameId, gameVersion, cheatVersion, error, language }) {
  const t = (fr, en) => language === 'fr' ? fr : en;
  const id = gameId.trim().toUpperCase();
  const observed = gameVersion.trim();
  const required = cheatVersion.trim();
  const result = [t('Données saisies manuellement : aucune version détectée automatiquement.', 'Manually entered data: no automatically detected version.')];
  if (!/^(CUSA|PPSA)\d{5}$/.test(id)) result.push(t('Identifiant manquant ou invalide : relevez l’ID exact du jeu et sa région.', 'Missing or invalid ID: find the exact game ID and region.'));
  if (!observed) result.push(t('Version du jeu inconnue : relevez la version du correctif installé avant de choisir un cheat.', 'Unknown game version: check the installed patch version before choosing a cheat.'));
  if (!required) result.push(t('Version requise par le cheat inconnue : consultez sa définition et son moteur.', 'Required cheat version is unknown: inspect its definition and engine.'));
  if (observed && required && observed !== required) result.push(t(`Incompatibilité probable : jeu ${observed}, cheat ${required}. Choisissez une définition correspondant à la même version et région.`, `Likely mismatch: game ${observed}, cheat ${required}. Choose a definition for the same version and region.`));
  if (observed && required && observed === required) result.push(t('Les versions saisies concordent, mais la région, le moteur et les offsets mémoire restent à vérifier. Cela ne valide pas la compatibilité.', 'Entered versions match, but region, engine and memory offsets still need verification. This does not validate compatibility.'));
  if (/unable to detect patch version/i.test(error)) result.push(t('Causes possibles : jeu non lancé, moteur incapable de lire le processus, définition absente ou version de correctif non reconnue. Lancez le jeu, vérifiez le moteur et sa compatibilité, puis consultez ses logs.', 'Possible causes: game not running, engine unable to read the process, missing definition or unrecognized patch version. Launch the game, verify engine compatibility, then inspect its logs.'));
  else if (error.trim()) result.push(t('Erreur non reconnue par cette analyse locale. Consultez les logs du moteur et sa documentation ; aucune cause certaine ne peut être établie.', 'Error not recognized by this local analysis. Inspect engine logs and documentation; no definitive cause can be established.'));
  result.push(t('JSON, MC4 et SHN : import et installation non implémentés. Ne modifiez pas les fichiers avant sauvegarde.', 'JSON, MC4 and SHN: import and installation are not implemented. Back up files before modifying them.'));
  return result;
}
