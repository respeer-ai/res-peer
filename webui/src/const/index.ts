export enum Apps {
  feedApp = 'dfb3274b80d749b0678ab741f1054f05a2377a7edfefdb21e000de6767d7ce47dff3ef30dce630fb5b05ed8a856249e2a4f47e4361fb9282f4e23fb823c3ff2a1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03050000000000000000000000',
  creditApp = 'b0780bb29703d15da8bc910cb01bb2a1b9d2a74946dc37a9a02eb956ecd339e1b0fa70cb43738443b535359d56062e12f8abc6c226b0ea7c258adbff7a6084921db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03010000000000000000000000',
  marketApp = '3ea45a0b5e158762224ca6536c7a209f6acd08f91b83ed11e51e9b8ed8af46a11efc175888c6f39a197a53a33daadf0f905f7bf36a272a5e1432a2125658be7e1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03070000000000000000000000',
  reviewApp = '82be78c9168524d563e704029aaaef47841a5e1c935d12a76e09aaba27e54a23fdda8a2711bf9162e799d3e6c37f2d1524c390587da9141f2764fd5b1931dc261db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03090000000000000000000000',
  activityApp = 'b2b20374716f1a04c276471888e083f9b2eca82a36bd7129ce678f4bdb73573b09ebfe4575e1615d25ea98e3fc67188283a74c721cdc0e930b2607dd14ceb6da1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030b0000000000000000000000',
  foundationApp = '1d7b6a04aa0c502ea07746d3910f8ede2113b87e4c15444130146916d14b95c81370e208896b040fe3627584abcb278bab97d9fd1a526e0bcabab7ff8e538e841db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03030000000000000000000000',
  blobGatewayApp = '569c64a918aaca6da66cdf6293123c416f978325e7c27b4e34d8b18d2b83c9c6dc559561f38cd893ba9381b098ab6eda2f9601fc1873e21e5ac159227cf75cd31db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030d0000000000000000000000',
  cpRegistryApp = '0bdaa100a4201d72795356022bf1aea528fe6730c404bc150fe6c07e088a66572394331adfaaaf65b6faf346c4ad7d9f606df8b66f1dfd1ec1b8b274c5f9b94c1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030f0000000000000000000000',
  copilotCpuApp = 'd8a5baa9da830d0f1969ac9492957f3355adff4e0b0bc741c9c81afd84d7e3091b7997a6a4d0e48340aaadadf1a827c63a0f4c0b3277f9bb5cd55926e3c3ef9a1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03110000000000000000000000',
  copilotGpuApp = 'd8a5baa9da830d0f1969ac9492957f3355adff4e0b0bc741c9c81afd84d7e3091b7997a6a4d0e48340aaadadf1a827c63a0f4c0b3277f9bb5cd55926e3c3ef9a1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03130000000000000000000000',
  illustratorCpuApp = 'f9f2cb8b533a98042ee7b88361da8121dda41eb8d72a216c4a1b831cee84b933df1589dbd241a987e890a9e9a17c8eec050100af37f5a226eb71cb6518ea4fb01db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03150000000000000000000000',
  illustratorGpuApp = 'f9f2cb8b533a98042ee7b88361da8121dda41eb8d72a216c4a1b831cee84b933df1589dbd241a987e890a9e9a17c8eec050100af37f5a226eb71cb6518ea4fb01db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03170000000000000000000000'
}

export const appIds = Object.values(Apps)

/// Chain which is the application originally deployed
export const appDeployChain = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03'
export const appDeployOwner = '7fc68b00dc83a95df7d6b41797ce0650421ce53c94929ba42d706c947e445e3b'

/// Port should be set with different service
export const port = '9081'
export const host = 'localhost'
