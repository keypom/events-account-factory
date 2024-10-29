import { CreationConfig, PremadeTicketData } from "../types";

export const GLOBAL_NETWORK = "mainnet";

export const SIGNER_ACCOUNT = "keypom.near";
export const CLEANUP_CONTRACT = false;
export const CREATION_CONFIG: CreationConfig = {
  deployContract: true,

  // TICKETS
  addTickets: false,
  premadeTickets: true,

  // ACCOUNTS
  createSponsors: true,
  createWorker: true,
  createAdmin: true,
  createTicketAdder: true,

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
