import { boot } from 'quasar/wrappers'
import VueDiff from 'vue-diff'
import plaintext from 'highlight.js/lib/languages/plaintext'
import 'vue-diff/dist/index.css'

VueDiff.hljs.registerLanguage('plaintext', plaintext)

// "async" is optional;
// more info on params: https://v2.quasar.dev/quasar-cli/boot-files
export default boot(({ app }) => {
  app.use(VueDiff, { componentName: 'VueDiff' })
})
