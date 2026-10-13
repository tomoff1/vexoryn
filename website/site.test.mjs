import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import vm from 'node:vm';

const source = await readFile(new URL('./site.js', import.meta.url), 'utf8');
async function run(responses) {
  const nodes = new Map();
  const calls = [];
  const makeNode = () => ({textContent:'',href:'',children:[],removed:[],classes:new Set(['disabled','hidden']),classList:{remove(name){this.owner.classes.delete(name);}},append(node){this.children.push(node);},removeAttribute(name){this.removed.push(name);}});
  const getNode = id => {if(!nodes.has(id)){const n=makeNode();n.classList.owner=n;nodes.set(id,n);}return nodes.get(id);};
  const context = vm.createContext({ URL, document:{getElementById:getNode,createElement:makeNode},fetch:async(url)=>{calls.push(url); const r=responses.shift(); if(!r)throw new Error('unexpected request');return {ok:r.status===200,status:r.status,json:async()=>r.body};}});
  // Avoid the automatic call so the test can await every operation.
  vm.runInContext(source.replace(/loadRelease\(\);\s*$/, ''), context);
  await vm.runInContext('loadRelease()',context);
  return {nodes,calls};
}
test('unconfigured site makes no release request and advertises no installer',async()=>{
  const {nodes,calls}=await run([{status:200,body:{repository:''}}]);
  assert.equal(calls.length,1);assert.equal(nodes.size,1);
});
test('missing release reports unavailable without producing download links',async()=>{
  const {nodes}=await run([{status:200,body:{repository:'VEXORYN/vexoryn'}},{status:404}]);
  assert.match(nodes.get('release-status').textContent,/Aucune version/);
  assert.equal(nodes.has('download'),false);
});
test('only actual same-repository release assets become download links',async()=>{
  const repo='VEXORYN/vexoryn';
  const {nodes}=await run([{status:200,body:{repository:repo}},{status:200,body:{tag_name:'v0.1.0',html_url:`https://github.com/${repo}/releases/tag/v0.1.0`,assets:[
    {name:'VEXORYN-setup.exe',browser_download_url:`https://github.com/${repo}/releases/download/v0.1.0/VEXORYN-setup.exe`},
    {name:'SHA256SUMS.txt',browser_download_url:`https://github.com/${repo}/releases/download/v0.1.0/SHA256SUMS.txt`},
    {name:'malicious.exe',browser_download_url:'https://example.com/malicious.exe'},
    {name:'other.exe',browser_download_url:'https://github.com/other/repository/releases/download/v0.1.0/other.exe'}
  ]}}]);
  assert.equal(nodes.get('assets').children.length,3);
  assert.equal(nodes.get('download').href,`https://github.com/${repo}/releases/download/v0.1.0/VEXORYN-setup.exe`);
  assert.equal(nodes.get('download').classes.has('disabled'),false);
});
test('invalid repository is rejected before contacting GitHub',async()=>{
  const {nodes,calls}=await run([{status:200,body:{repository:'../evil'}}]);
  assert.equal(calls.length,1);assert.match(nodes.get('release-status').textContent,/Dépôt invalide/);
});
