import test from 'node:test';
import assert from 'node:assert/strict';
import { isPrivateIPv4, isPrivateSubnet24, derivePrivateSubnet, analyzeCheatFacts } from '../src/diagnostics.mjs';

test('IPv4 validation rejects public, ambiguous, malformed and hostname destinations', () => {
  for (const ip of ['8.8.8.8', '169.254.1.2', '172.15.0.1', '172.32.0.1', '192.168.1.256', '192.168.01.1', 'localhost', '192.168.1', '10.0.0.1:21', ' 10.0.0.1', '127.0.0.1', '']) assert.equal(isPrivateIPv4(ip), false, ip);
  for (const ip of ['10.0.0.1', '192.168.1.50', '172.16.0.1', '172.31.255.254']) assert.equal(isPrivateIPv4(ip), true, ip);
});

test('discovery scope is one canonical RFC1918 /24, never loopback or public', () => {
  for (const subnet of ['10.0.0.0/24', '172.31.4.0/24', '192.168.1.0/24']) assert.equal(isPrivateSubnet24(subnet), true, subnet);
  for (const subnet of ['127.0.0.0/24', '8.8.8.0/24', '192.168.1.1/24', '10.0.0.0/8', '192.168.1.0/16', '10.00.0.0/24']) assert.equal(isPrivateSubnet24(subnet), false, subnet);
  assert.equal(derivePrivateSubnet('192.168.42.17'), '192.168.42.0/24');
  assert.equal(derivePrivateSubnet('127.0.0.1'), '');
  assert.equal(derivePrivateSubnet('8.8.8.8'), '');
});

const facts = { gameId: 'PPSA12345', gameVersion: '01.000.000', cheatVersion: '01.000.000', error: '', language: 'en' };
test('matching cheat versions never certify compatibility or invent detected telemetry', () => {
  const result = analyzeCheatFacts(facts).join('\n');
  assert.match(result, /no automatically detected version/);
  assert.match(result, /does not validate compatibility/);
  assert.match(result, /region, engine and memory offsets/);
});
test('patch failure explains hypotheses and unknown data instead of asserting a cause', () => {
  const result = analyzeCheatFacts({ ...facts, gameVersion: '', cheatVersion: '', error: 'Unable to detect patch version' }).join('\n');
  assert.match(result, /Unknown game version/);
  assert.match(result, /Required cheat version is unknown/);
  assert.match(result, /Possible causes/);
  assert.doesNotMatch(result, /Likely mismatch/);
});
test('different versions surface exact observations; unknown errors stay inconclusive', () => {
  const result = analyzeCheatFacts({ ...facts, gameVersion: '02.000.000', error: 'Unmapped error 42' }).join('\n');
  assert.match(result, /game 02\.000\.000, cheat 01\.000\.000/);
  assert.match(result, /no definitive cause/);
});
test('French analysis validates game identity and normalizes whitespace only', () => {
  const result = analyzeCheatFacts({ ...facts, language: 'fr', gameId: 'unknown', gameVersion: ' 01.000.000 ', cheatVersion: '01.000.000' }).join('\n');
  assert.match(result, /Identifiant manquant ou invalide/);
  assert.match(result, /versions saisies concordent/);
  assert.doesNotMatch(result, /Incompatibilité probable/);
});
