import { CreationConfig, PremadeTicketData } from "../types";

export const GLOBAL_NETWORK = "testnet";

export const SIGNER_ACCOUNT = "benjiman.testnet";
export const FREEZE_CONTRACT = false;
export const CLEANUP_CONTRACT = false;
export const CREATION_CONFIG: CreationConfig = {
  deployContract: false,

  // TICKETS
  addTickets: false,
  premadeTickets: false,

  // ACCOUNTS
  createSponsors: true,
  createWorker: false,
  createAdmin: false,
  createTicketAdder: false,

  nftDrops: false,
  tokenDrops: false,
  scavDrops: false,
  multichainDrops: false,
};

export const SITE_BASE_URL = "https://development.keypom-events-app.pages.dev";
export const EXISTING_FACTORY = `1730317152765-factory.testnet`;
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
  {
    name: "Test User 4",
    email: "foo",
  },
];
