import { boot } from 'quasar/wrappers'
import PrimeVue from 'primevue/config'
import Aura from '@primevue/themes/aura'

// "async" is optional;
// more info on params: https://v2.quasar.dev/quasar-cli/boot-files
export default boot(({ app }) => {
  app.use(PrimeVue, {
    theme: {
      // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
      preset: Aura
    }
  })
})
