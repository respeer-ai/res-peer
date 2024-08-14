export enum Apps {
  feedApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030600000000000000000000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03080000000000000000000000',
  creditApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030000000000000000000000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03020000000000000000000000',
  marketApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030900000000000000000000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030b0000000000000000000000',
  reviewApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030c00000000000000000000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030e0000000000000000000000',
  activityApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030f00000000000000020000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03110000000000000000000000',
  foundationApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030300000000000000000000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03050000000000000000000000',
  blobGatewayApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d031200000000000000000000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03140000000000000000000000',
  cpRegistryApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d031500000000000000000000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03170000000000000000000000',
  copilotApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d031b00000000000000000000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d031d0000000000000000000000',
  illustratorApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d031800000000000000000000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d031a0000000000000000000000'
}

export const appIds = Object.values(Apps)

/// Chain which is the application originally deployed
export const appDeployChain = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03'
export const appDeployOwner = 'ad9f9b40b47d35e369ae32b5f6c820b81881f6842de1ecda0eb347f0d235427f'

/// Port should be set with different service
export const port = '9080'
export const host = 'localhost'
