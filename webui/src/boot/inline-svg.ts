import { boot } from 'quasar/wrappers'
import InlineSvg from 'vue-inline-svg'

// "async" is optional;
// more info on params: https://v2.quasar.dev/quasar-cli/boot-files
export default boot(({ app }) => {
  app.component('InlineSvg', InlineSvg)
})
