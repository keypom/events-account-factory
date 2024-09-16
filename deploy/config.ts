export const GLOBAL_NETWORK = "testnet";

export const SIGNER_ACCOUNT = "benjiman.testnet";
export const CREATION_CONFIG = {
  deployContract: false,

  // TICKETS
  addTickets: false,
  premadeTickets: false,

  // ACCOUNTS
  createSponsors: false,
  createWorker: false,
  createAdmin: true,

  premadeDrops: false,
};
export const NUM_TICKETS_TO_ADD = 10;

export const TICKET_URL_BASE =
  "https://2930bf5d.keypom-redacted-app.pages.dev/tickets/ticket/ga_pass#";
export const EXISTING_FACTORY = `1726258983645-factory.testnet`;
export const ADMIN_ACCOUNTS = [SIGNER_ACCOUNT];

export const TICKET_DATA = {
  ga_pass: {
    startingNearBalance: "0.01",
    startingTokenBalance: "50",
    accountType: "Basic",
  },
};
export const SPONSOR_DATA = [
  {
    accountName: "proximity",
    startingNearBalance: "0.01",
    startingTokenBalance: "50",
    accountType: "Sponsor",
  },
];
export const PREMADE_TICKET_DATA = [
  {
    name: "Jake",
    email: "",
  },
  {
    name: "Kiana",
    email: "foo",
  },
  {
    name: "Min",
    email: "foo",
  },
  {
    name: "Benji",
    email: "foo",
  },
  {
    name: "David",
    email: "foo",
  },
];

export const PREMADE_TOKEN_DROP_DATA = [
  {
    drop_data: {
      name: "10 Tokens",
    },
    token_amount: "10",
  },
  {
    drop_data: {
      name: "20 Tokens",
    },
    token_amount: "20",
  },
  {
    drop_data: {
      name: "25 Tokens",
    },
    token_amount: "25",
  },
];

export const PREMADE_NFT_DROP_DATA = [
  {
    drop_data: {
      name: "Illia's Talk POAP",
    },
    nft_metadata: {
      title: "Illia Talk POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
  {
    drop_data: {
      name: "Opening Ceremony POAP",
    },
    nft_metadata: {
      title: "Opening Ceremony POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
  {
    drop_data: {
      name: "Closing Ceremony POAP",
    },
    nft_metadata: {
      title: "Closing Ceremony POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
  {
    drop_data: {
      name: "Keypom Booth POAP",
    },
    nft_metadata: {
      title: "Keypom Booth POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
  {
    drop_data: {
      name: "Nuffle Waffles POAP",
    },
    nft_metadata: {
      title: "Nuffle Waffles POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
  {
    drop_data: {
      name: "Proximity Booth POAP",
    },
    nft_metadata: {
      title: "Proximity Booth POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
  {
    drop_data: {
      name: "Snowden Talk POAP",
    },
    nft_metadata: {
      title: "Snowden Talk POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
  {
    drop_data: {
      name: "Eric Winer Talk POAP",
    },
    nft_metadata: {
      title: "Eric Winer Talk POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
];

export const PREMADE_SCAVENGER_HUNTS = [
  {
    drop_data: {
      name: "Official Scavenger Hunt",
      scavenger_hunt: [
        {
          piece: "1",
          description: "Find at location 1",
        },
        {
          piece: "2",
          description: "Find at location 2",
        },
        {
          piece: "3",
          description: "Find at location 3",
        },
        {
          piece: "4",
          description: "Find at location 4",
        },
      ],
    },
    nft_metadata: {
      title: "Redacted Scavenger Hunt POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
  {
    drop_data: {
      name: "Keypom Scavenger Hunt",
      scavenger_hunt: [
        {
          piece: "1",
          description: "Find at location 1",
        },
        {
          piece: "2",
          description: "Find at location 2",
        },
        {
          piece: "3",
          description: "Find at location 3",
        },
        {
          piece: "4",
          description: "Find at location 4",
        },
      ],
    },
    nft_metadata: {
      title: "Keypom Scavenger Hunt POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
  {
    drop_data: {
      name: "Proximity Scavenger Hunt",
      scavenger_hunt: [
        {
          piece: "1",
          description: "Find at location 1",
        },
        {
          piece: "2",
          description: "Find at location 2",
        },
        {
          piece: "3",
          description: "Find at location 3",
        },
        {
          piece: "4",
          description: "Find at location 4",
        },
      ],
    },
    nft_metadata: {
      title: "Proximity Scavenger Hunt POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
  {
    drop_data: {
      name: "Nuffle Labs Scavenger Hunt",
      scavenger_hunt: [
        {
          piece: "1",
          description: "Find at location 1",
        },
        {
          piece: "2",
          description: "Find at location 2",
        },
        {
          piece: "3",
          description: "Find at location 3",
        },
        {
          piece: "4",
          description: "Find at location 4",
        },
      ],
    },
    nft_metadata: {
      title: "Nuffle Labs Scavenger Hunt POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
  {
    drop_data: {
      name: "DevHub Scavenger Hunt",
      scavenger_hunt: [
        {
          piece: "1",
          description: "Find at location 1",
        },
        {
          piece: "2",
          description: "Find at location 2",
        },
        {
          piece: "3",
          description: "Find at location 3",
        },
        {
          piece: "4",
          description: "Find at location 4",
        },
      ],
    },
    token_amount: "25",
  },
  {
    drop_data: {
      name: "NEAR Horizons Scavenger Hunt",
      scavenger_hunt: [
        {
          piece: "1",
          description: "Find at location 1",
        },
        {
          piece: "2",
          description: "Find at location 2",
        },
        {
          piece: "3",
          description: "Find at location 3",
        },
        {
          piece: "4",
          description: "Find at location 4",
        },
      ],
    },
    token_amount: "15",
  },
  {
    drop_data: {
      name: "NEAR AI Scavenger Hunt",
      scavenger_hunt: [
        {
          piece: "1",
          description: "Find at location 1",
        },
        {
          piece: "2",
          description: "Find at location 2",
        },
        {
          piece: "3",
          description: "Find at location 3",
        },
        {
          piece: "4",
          description: "Find at location 4",
        },
      ],
    },
    token_amount: "10",
  },
];
