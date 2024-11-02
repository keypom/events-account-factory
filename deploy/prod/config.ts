import { CreationConfig, PremadeTicketData } from "../types";

export const GLOBAL_NETWORK = "mainnet";

export const SIGNER_ACCOUNT = "keypom.near";
export const FREEZE_CONTRACT = false;
export const CLEANUP_CONTRACT = false;
export const CREATION_CONFIG: CreationConfig = {
  deployContract: false,

  // TICKETS
  addTickets: false,
  premadeTickets: false,

  // ACCOUNTS
  createSponsors: false,
  createWorker: false,
  createAdmin: false,
  createTicketAdder: false,

  nftDrops: false,
  tokenDrops: false,
  scavDrops: false,
  multichainDrops: false,
};
export const NUM_TICKETS_TO_ADD = 10;

export const SITE_BASE_URL = "https://app.redactedbangkok.ai";
export const EXISTING_FACTORY = `redacted2024.near`;
export const ADMIN_ACCOUNTS = [SIGNER_ACCOUNT];

export const PREMADE_TICKET_DATA: PremadeTicketData = [
  {
    name: "Test User 1",
    email: "",
  },
  {
    name: "Test User 2",
    email: "foo",
  },
  {
    name: "Test User 3",
    email: "foo",
  },
];
