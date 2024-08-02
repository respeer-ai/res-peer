<div>
  <div :style='{width: "720px", margin: "0 auto"}' class='flex justify-center items-center row'>
    <q-img
      src='~assets/ResPeer.png' width='160px' fit='contain' class='cursor-pointer'
      @click='onLogoClick'
    />
    <q-space />
    <q-icon
      name='store' size='24px' :color='tab == "store" ? "green" : "black"' class='cursor-pointer'
      @click='onNFTMarketClick'
    />
    <q-btn
      flat rounded class='bg-red-2'
      @click='onLoginClick'
      :style='{marginLeft: "8px"}'
    >
      <q-menu
        v-if='account?.length'
        :style='{padding: "24px"}'
        anchor='bottom right'
        self='top right'
      >
        <q-card flat>
          <div class='text-brown-10 row flex justify-center items-center' :style='{margin: "24px 0 0 0", lineHeight: "48px"}'>
            <q-space />
            <div class='text-bold' :style='{fontSize: "36px", marginLeft: "8px"}'>
              {{ Number(accountBalance).toFixed(4) }}
            </div>
            <q-space />
          </div>
          <div class='row' :style='{margin: "8px 0 48px 0", lineHeight: "24px"}'>
            <q-space />
            <q-img
              src='https://avatars.githubusercontent.com/u/107513858?s=48&v=4'
              width='24px'
              height='24px'
            />
            <div class='text-brown-6' :style='{fontSize: "24px", marginLeft: "8px"}'>
              TLINERA
            </div>
            <q-space />
          </div>
          <q-separator :style='{margin: "8px 0"}' />
          <div class='row flex justify-center items-center'>
            <div :style='{width: "48px"}'>
              <q-icon name='person' size='24px' class='text-red-6' />
            </div>
            <div>
              <div class='text-grey-6'>
                Address
              </div>
              <div class='row'>
                <div class='text-bold'>
                  {{ shortid.shortId(account, 16) }}
                </div>
                <div :style='{marginLeft: "8px"}' class='cursor-pointer'>
                  <q-icon name='content_copy' size='16px' class='text-grey-6' />
                </div>
              </div>
              <div class='text-grey-6'>
                {{ Number(accountBalance).toFixed(4) }}
              </div>
            </div>
          </div>
          <q-separator :style='{margin: "8px 0"}' />
          <div class='row flex justify-center items-center'>
            <div :style='{width: "48px"}'>
              <q-icon name='link' size='24px' class='text-red-6' />
            </div>
            <div>
              <div class='text-grey-6'>
                Microchain
              </div>
              <div class='row'>
                <div class='text-bold'>
                  {{ shortid.shortId(chainId, 16) }}
                </div>
                <div :style='{marginLeft: "8px"}' class='cursor-pointer'>
                  <q-icon name='content_copy' size='16px' class='text-grey-6' />
                </div>
              </div>
              <div class='text-grey-6'>
                {{ Number(chainBalance).toFixed(4) }}
              </div>
            </div>
          </div>
          <q-btn
            flat rounded class='bg-red-2 full-width'
            @click='onLogoutClick'
            label='Logout'
            :style='{margin: "24px 0 0 0"}'
          />
          <div class='text-grey-6 text-center' :style='{margin: "8px 0 16px 0"}'>
            Powered by CheCko
          </div>
        </q-card>
      </q-menu>
      <q-img src='~assets/CheCko.png' width='24px' />
      <div :style='{margin: "2px 0 0 8px"}' class='text-brown-8 text-bold'>
        {{ account?.length ? shortid.shortId(account, 4) : 'Login' }}
      </div>
    </q-btn>
    <q-icon
      name='local_activity' size='24px' :color='tab == "activity" ? "green" : "black"' class='cursor-pointer'
      :style='{marginLeft: "8px"}'
      @click='onActivityClick'
    />
    <q-icon
      v-if='account?.length'
      name='dashboard' size='24px' :color='tab == "dashboard" ? "green" : "black"' class='cursor-pointer'
      :style='{marginLeft: "8px"}'
      @click='onDashboardClick'
    />
  </div>
</div>
