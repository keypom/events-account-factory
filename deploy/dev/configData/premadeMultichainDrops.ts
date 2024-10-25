import { PremadeMultichainDrops } from "../../types";

const BASE_SEPOLIA_CONTRACT_ADDRESS =
  "0xCeb40Ce9979f2F044031759cCA5a3e2C3fc04c42";
const BSC_TESTNET_CONTRACT_ADDRESS =
  "0xc3aFb0756eF0Cbd07Bb0fD54817F3d44fF23EDF6";
const OPTIMISM_SEPOLIA_CONTRACT_ADDRESS =
  "0x7E3192C399b06A547fB3C849aFb6E79Bf9EDBAd1";
const ARBITRUM_SEPOLIA_CONTRACT_ADDRESS =
  "0x7E3192C399b06A547fB3C849aFb6E79Bf9EDBAd1";
const ETH_SEPOLIA_BUILDBEAR_CONTRACT_ADDRESS =
  "0x3B631fc7c985B66BF8fE82A2cf32233CADE28ced";
const POLYGON_AMOX_CONTRACT_ADDRESS =
  "0xD6a277D5E38DB435C008Bc604EfA89F9777DB586";

const BASE_SEPOLIA = 84532;
const BSC_TESTNET = 97;
const OPTIMISM_SEPOLIA = 11155420;
const ARBITRUM_SEPOLIA = 421614;
const ETH_SEPOLIA_BUILDBEAR = 20665;
const POLYGON_AMOY = 80002;

// each chain has different contract address, chain id, and needs chain name appended to drop_data.name and nft_metadata.title
const MULTICHAIN_DROP_SHARED_DATA: PremadeMultichainDrops = [
  // Broccoli
  {
    drop_data: {
      image: "bafkreiboe7owyumlwghxwa56g5atdshbrloel5b25whjoiw46x5e74yeny",
      name: "Broccoli Dogs POAP",
    },
    nft_metadata: {
      title: "Multichain Test Drop",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media: "bafkreiboe7owyumlwghxwa56g5atdshbrloel5b25whjoiw46x5e74yeny",
    },
    multichain_metadata: {
      chain_id: 84532,
      contract_id: "0xD6B95F11213cC071B982D717721B1aC7Bc628d46",
      series_id: 1,
    },
  },
  // Wikipedia
  {
    drop_data: {
      image: "bafybeierpmvbgnmhnbj5c3fiqtxa77jqzqltsitrwp3wrlk5wupjsfwooe",
      name: "Wikipedia Dog POAP",
    },
    nft_metadata: {
      title: "Multichain Test Drop",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media: "bafybeierpmvbgnmhnbj5c3fiqtxa77jqzqltsitrwp3wrlk5wupjsfwooe",
    },
    multichain_metadata: {
      chain_id: 84532,
      contract_id: "0xD6B95F11213cC071B982D717721B1aC7Bc628d46",
      series_id: 2,
    },
  },
  // Surfer
  {
    drop_data: {
      image: "bafkreif2yab3wb4rooxsc56xufkgxjf6uekr3mg72j7lj2mowq554mwqaq",
      name: "Surfer Dog POAP",
    },
    nft_metadata: {
      title: "Multichain Test Drop",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media: "bafkreif2yab3wb4rooxsc56xufkgxjf6uekr3mg72j7lj2mowq554mwqaq",
    },
    multichain_metadata: {
      chain_id: 84532,
      contract_id: "0xD6B95F11213cC071B982D717721B1aC7Bc628d46",
      series_id: 3,
    },
  },
  // California
  {
    drop_data: {
      image: "bafybeifrcuygwadhrsowc4ngbs2t6n3gx2kwununa27v7wplr5ou2cjfka",
      name: "California Dog POAP",
    },
    nft_metadata: {
      title: "Multichain Test Drop",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media: "bafybeifrcuygwadhrsowc4ngbs2t6n3gx2kwununa27v7wplr5ou2cjfka",
    },
    multichain_metadata: {
      chain_id: 84532,
      contract_id: "0xD6B95F11213cC071B982D717721B1aC7Bc628d46",
      series_id: 4,
    },
  },
  // Union
  {
    drop_data: {
      image: "bafkreibefi7i67edtrvmj2zn74z7viqcirb7w5ui6cpolp6xhcqha2666y",
      name: "Union Dog POAP",
    },
    nft_metadata: {
      title: "Multichain Test Drop",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media: "bafkreibefi7i67edtrvmj2zn74z7viqcirb7w5ui6cpolp6xhcqha2666y",
    },
    multichain_metadata: {
      chain_id: 84532,
      contract_id: "0xD6B95F11213cC071B982D717721B1aC7Bc628d46",
      series_id: 5,
    },
  },
  // Watermelon
  {
    drop_data: {
      image: "bafkreifgzss27l5nvxy5dqt7idz2ry2l2d6almkerii5btdgz2fozgmsie",
      name: "Watermelon Dog POAP",
    },
    nft_metadata: {
      title: "Multichain Test Drop",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media: "bafkreifgzss27l5nvxy5dqt7idz2ry2l2d6almkerii5btdgz2fozgmsie",
    },
    multichain_metadata: {
      chain_id: 84532,
      contract_id: "0xD6B95F11213cC071B982D717721B1aC7Bc628d46",
      series_id: 6,
    },
  },
  // Magic
  {
    drop_data: {
      image: "bafybeicrn2ix4xa5crz7cjjthvp7fukju7ot3v5qfyvcvnjtrosrb5cube",
      name: "Magic Dog POAP",
    },
    nft_metadata: {
      title: "Multichain Test Drop",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media: "bafybeicrn2ix4xa5crz7cjjthvp7fukju7ot3v5qfyvcvnjtrosrb5cube",
    },
    multichain_metadata: {
      chain_id: 84532,
      contract_id: "0xD6B95F11213cC071B982D717721B1aC7Bc628d46",
      series_id: 7,
    },
  },
]

const BASE_SEPOLIA_DROPS: PremadeMultichainDrops =
  MULTICHAIN_DROP_SHARED_DATA.map((drop) => {
    return {
      drop_data: {
        ...drop.drop_data,
        name: `Base Sepolia ${drop.drop_data.name}`,
      },
      nft_metadata: {
        ...drop.nft_metadata,
        title: `Base Sepolia ${drop.nft_metadata.title}`,
      },
      multichain_metadata: {
        ...drop.multichain_metadata,
        chain_id: BASE_SEPOLIA,
        contract_id: BASE_SEPOLIA_CONTRACT_ADDRESS,
      },
    };
  });

const BSC_TESTNET_DROPS: PremadeMultichainDrops =
  MULTICHAIN_DROP_SHARED_DATA.map((drop) => {
    return {
      drop_data: {
        ...drop.drop_data,
        name: `BSC Testnet ${drop.drop_data.name}`,
      },
      nft_metadata: {
        ...drop.nft_metadata,
        title: `BSC Testnet ${drop.nft_metadata.title}`,
      },
      multichain_metadata: {
        ...drop.multichain_metadata,
        chain_id: BSC_TESTNET,
        contract_id: BSC_TESTNET_CONTRACT_ADDRESS,
      },
    };
  });

const OPTIMISM_SEPOLIA_DROPS: PremadeMultichainDrops =
  MULTICHAIN_DROP_SHARED_DATA.map((drop) => {
    return {
      drop_data: {
        ...drop.drop_data,
        name: `Optimism Sepolia ${drop.drop_data.name}`,
      },
      nft_metadata: {
        ...drop.nft_metadata,
        title: `Optimism Sepolia ${drop.nft_metadata.title}`,
      },
      multichain_metadata: {
        ...drop.multichain_metadata,
        chain_id: OPTIMISM_SEPOLIA,
        contract_id: OPTIMISM_SEPOLIA_CONTRACT_ADDRESS,
      },
    };
  });

const ARBITRUM_SEPOLIA_DROPS: PremadeMultichainDrops =
  MULTICHAIN_DROP_SHARED_DATA.map((drop) => {
    return {
      drop_data: {
        ...drop.drop_data,
        name: `Arbitrum Sepolia ${drop.drop_data.name}`,
      },
      nft_metadata: {
        ...drop.nft_metadata,
        title: `Arbitrum Sepolia ${drop.nft_metadata.title}`,
      },
      multichain_metadata: {
        ...drop.multichain_metadata,
        chain_id: ARBITRUM_SEPOLIA,
        contract_id: ARBITRUM_SEPOLIA_CONTRACT_ADDRESS,
      },
    };
  });

const ETH_SEPOLIA_BUILDBEAR_DROPS: PremadeMultichainDrops =
  MULTICHAIN_DROP_SHARED_DATA.map((drop) => {
    return {
      drop_data: {
        ...drop.drop_data,
        name: `ETH Sepolia Buildbear ${drop.drop_data.name}`,
      },
      nft_metadata: {
        ...drop.nft_metadata,
        title: `ETH Sepolia Buildbear ${drop.nft_metadata.title}`,
      },
      multichain_metadata: {
        ...drop.multichain_metadata,
        chain_id: ETH_SEPOLIA_BUILDBEAR,
        contract_id: ETH_SEPOLIA_BUILDBEAR_CONTRACT_ADDRESS,
      },
    };
  });

const POLYGON_AMOY_DROPS: PremadeMultichainDrops =
  MULTICHAIN_DROP_SHARED_DATA.map((drop) => {
    return {
      drop_data: {
        ...drop.drop_data,
        name: `Polygon Amoy ${drop.drop_data.name}`,
      },
      nft_metadata: {
        ...drop.nft_metadata,
        title: `Polygon Amoy ${drop.nft_metadata.title}`,
      },
      multichain_metadata: {
        ...drop.multichain_metadata,
        chain_id: POLYGON_AMOY,
        contract_id: POLYGON_AMOX_CONTRACT_ADDRESS,
      },
    };
  });

export const PREMADE_MULTICHAIN_DROPS: PremadeMultichainDrops = [
  ...BASE_SEPOLIA_DROPS,
  ...BSC_TESTNET_DROPS,
  ...OPTIMISM_SEPOLIA_DROPS,
  ...ARBITRUM_SEPOLIA_DROPS,
  ...ETH_SEPOLIA_BUILDBEAR_DROPS,
  ...POLYGON_AMOY_DROPS,
];

// console.log(PREMADE_MULTICHAIN_DROPS)

/*
const MULTICHAIN_DROP_SHARED_DATA: PremadeMultichainDrops = [
  // Broccoli
  {
    drop_data: {
      image: "bafkreiboe7owyumlwghxwa56g5atdshbrloel5b25whjoiw46x5e74yeny",
      name: "Broccoli Dogs POAP",
    },
    nft_metadata: {
      title: "Multichain Test Drop",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media: "bafkreiboe7owyumlwghxwa56g5atdshbrloel5b25whjoiw46x5e74yeny",
    },
    multichain_metadata: {
      chain_id: 84532,
      contract_id: "0xD6B95F11213cC071B982D717721B1aC7Bc628d46",
      series_id: 1,
    },
  },
  // Wikipedia
  {
    drop_data: {
      image: "bafybeierpmvbgnmhnbj5c3fiqtxa77jqzqltsitrwp3wrlk5wupjsfwooe",
      name: "Wikipedia Dog POAP",
    },
    nft_metadata: {
      title: "Multichain Test Drop",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media: "bafybeierpmvbgnmhnbj5c3fiqtxa77jqzqltsitrwp3wrlk5wupjsfwooe",
    },
    multichain_metadata: {
      chain_id: 84532,
      contract_id: "0xD6B95F11213cC071B982D717721B1aC7Bc628d46",
      series_id: 2,
    },
  },
  // Surfer
  {
    drop_data: {
      image: "bafkreif2yab3wb4rooxsc56xufkgxjf6uekr3mg72j7lj2mowq554mwqaq",
      name: "Surfer Dog POAP",
    },
    nft_metadata: {
      title: "Multichain Test Drop",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media: "bafkreif2yab3wb4rooxsc56xufkgxjf6uekr3mg72j7lj2mowq554mwqaq",
    },
    multichain_metadata: {
      chain_id: 84532,
      contract_id: "0xD6B95F11213cC071B982D717721B1aC7Bc628d46",
      series_id: 3,
    },
  },
  // California
  {
    drop_data: {
      image: "bafybeifrcuygwadhrsowc4ngbs2t6n3gx2kwununa27v7wplr5ou2cjfka",
      name: "California Dog POAP",
    },
    nft_metadata: {
      title: "Multichain Test Drop",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media: "bafybeifrcuygwadhrsowc4ngbs2t6n3gx2kwununa27v7wplr5ou2cjfka",
    },
    multichain_metadata: {
      chain_id: 84532,
      contract_id: "0xD6B95F11213cC071B982D717721B1aC7Bc628d46",
      series_id: 4,
    },
  },
  // Union
  {
    drop_data: {
      image: "bafkreibefi7i67edtrvmj2zn74z7viqcirb7w5ui6cpolp6xhcqha2666y",
      name: "Union Dog POAP",
    },
    nft_metadata: {
      title: "Multichain Test Drop",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media: "bafkreibefi7i67edtrvmj2zn74z7viqcirb7w5ui6cpolp6xhcqha2666y",
    },
    multichain_metadata: {
      chain_id: 84532,
      contract_id: "0xD6B95F11213cC071B982D717721B1aC7Bc628d46",
      series_id: 5,
    },
  },
  // Watermelon
  {
    drop_data: {
      image: "bafkreifgzss27l5nvxy5dqt7idz2ry2l2d6almkerii5btdgz2fozgmsie",
      name: "Watermelon Dog POAP",
    },
    nft_metadata: {
      title: "Multichain Test Drop",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media: "bafkreifgzss27l5nvxy5dqt7idz2ry2l2d6almkerii5btdgz2fozgmsie",
    },
    multichain_metadata: {
      chain_id: 84532,
      contract_id: "0xD6B95F11213cC071B982D717721B1aC7Bc628d46",
      series_id: 6,
    },
  },
  // Magic
  {
    drop_data: {
      image: "bafybeicrn2ix4xa5crz7cjjthvp7fukju7ot3v5qfyvcvnjtrosrb5cube",
      name: "Magic Dog POAP",
    },
    nft_metadata: {
      title: "Multichain Test Drop",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media: "bafybeicrn2ix4xa5crz7cjjthvp7fukju7ot3v5qfyvcvnjtrosrb5cube",
    },
    multichain_metadata: {
      chain_id: 84532,
      contract_id: "0xD6B95F11213cC071B982D717721B1aC7Bc628d46",
      series_id: 7,
    },
  },
]
*/

