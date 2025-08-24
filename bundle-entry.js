import { AuthClient } from '@dfinity/auth-client';
import { HttpAgent, Actor } from '@dfinity/agent';
import { IDL } from '@dfinity/candid';
import { Principal } from '@dfinity/principal';

window.AuthClient = AuthClient;
window.HttpAgent = HttpAgent;
window.Actor = Actor;
window.IDL = IDL;
window.Principal = Principal;

console.log('ICP libraries loaded:', {
  AuthClient: !!window.AuthClient,
  HttpAgent: !!window.HttpAgent,
  Actor: !!window.Actor,
  IDL: !!window.IDL,
  Principal: !!window.Principal
});