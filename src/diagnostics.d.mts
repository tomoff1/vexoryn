export function isPrivateIPv4(ip: string): boolean;
export function isPrivateSubnet24(subnet: string): boolean;
export function derivePrivateSubnet(ip: string): string;
export function analyzeCheatFacts(facts: { gameId: string; gameVersion: string; cheatVersion: string; error: string; language: 'fr' | 'en' }): string[];
