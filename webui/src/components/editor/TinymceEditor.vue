<template>
  <div>
    <editor
      :init='editorInit'
      @input='handleEditorInput'
      v-model='internalValue'
    />
  </div>
</template>

<script setup lang='ts'>
import { ref, watch, defineProps, withDefaults, defineEmits, onMounted } from 'vue'
import Editor from '@tinymce/tinymce-vue'
import { copilotIcon } from 'src/assets'

import 'tinymce/tinymce.min.js'
import 'tinymce/plugins/accordion/plugin.min.js'
import 'tinymce/plugins/advlist/plugin.min.js'
import 'tinymce/plugins/anchor/plugin.min.js'
import 'tinymce/plugins/autolink/plugin.min.js'
import 'tinymce/plugins/autoresize/plugin.min.js'
import 'tinymce/plugins/autosave/plugin.min.js'
import 'tinymce/plugins/charmap/plugin.min.js'
import 'tinymce/plugins/code/plugin.min.js'
import 'tinymce/plugins/codesample/plugin.min.js'
import 'tinymce/plugins/directionality/plugin.min.js'
import 'tinymce/plugins/emoticons/plugin.min.js'
import 'tinymce/plugins/emoticons/js/emojis.js'
import 'tinymce/plugins/fullscreen/plugin.min.js'
import 'tinymce/plugins/help/plugin.min.js'
import 'tinymce/plugins/image/plugin.min.js'
import 'tinymce/plugins/importcss/plugin.min.js'
import 'tinymce/plugins/insertdatetime/plugin.min.js'
import 'tinymce/plugins/link/plugin.min.js'
import 'tinymce/plugins/lists/plugin.min.js'
import 'tinymce/plugins/media/plugin.min.js'
import 'tinymce/plugins/nonbreaking/plugin.min.js'
import 'tinymce/plugins/pagebreak/plugin.min.js'
import 'tinymce/plugins/preview/plugin.min.js'
import 'tinymce/plugins/quickbars/plugin.min.js'
import 'tinymce/plugins/save/plugin.min.js'
import 'tinymce/plugins/searchreplace/plugin.min.js'
import 'tinymce/plugins/table/plugin.min.js'
import 'tinymce/plugins/template/plugin.min.js'
import 'tinymce/plugins/visualblocks/plugin.min.js'
import 'tinymce/plugins/visualchars/plugin.min.js'
import 'tinymce/plugins/wordcount/plugin.min.js'
import 'tinymce/plugins/help/js/i18n/keynav/en.js'

import 'tinymce/themes/silver/theme.js'
import 'tinymce/models/dom/model.js'
import 'tinymce/icons/default/icons.js'

import { Cookies } from 'quasar'

const apiURL = ref('')

const uploadMedia = (resolve: (value: string) => void, reject: (value: string) => void, filename: string, base64: string) => {
  console.log('Upload media', resolve, reject, filename, base64)
}

const initApiURL = () => {
  const devURL = 'https://api.development.npool.top/api'
  apiURL.value = process.env.DEV ? devURL : Cookies.get('X-Base-URL')
}

interface CustomBlob extends Blob {
  name: string
}

const exampleImageUploadHandler = (blobInfo: { blob: () => CustomBlob }) => new Promise((resolve, reject) => {
  const reader = new FileReader()
  reader.onload = (e: ProgressEvent<FileReader>) => {
    const base64Image = e.target?.result as string
    const base64String = base64Image.split(',')[1]
    uploadMedia(resolve, reject, fileName, base64String)
  }
  const blob = blobInfo.blob()
  const fileName = blob.name
  reader.readAsDataURL(blobInfo.blob())
})

interface Props {
  modelValue: string,
  plugins: string | Array<unknown>,
  toolbar: string | Array<unknown>
}
const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  plugins: 'preview searchreplace autolink directionality visualblocks visualchars ' +
          'fullscreen image link media code codesample table charmap pagebreak nonbreaking ' +
          'anchor insertdatetime advlist lists wordcount help emoticons autosave autoresize',
  toolbar: 'code undo redo restoredraft | fullscreen | cut copy | ' +
          'forecolor backcolor bold italic underline strikethrough link anchor | ' +
          'alignleft aligncenter alignright alignjustify outdent indent | ' +
          'styleselect formatselect fontselect fontsizeselect | bullist numlist | ' +
          'blockquote subscript superscript removeformat | ' +
          'table image media charmap emoticons hr pagebreak insertdatetime print preview | ' +
          'bdmap indent2em lineheight axupimgs'
})

const emit = defineEmits<{(ev: 'update:modelValue', modelValue: string): void}>()

const editorInit = ref({
  height: 500,
  min_height: 500,
  max_height: 500,
  menubar: 'edit view insert format tools table',
  plugins: props.plugins,
  toolbar: props.toolbar,
  skin_url: '/tinymce/skins/ui/oxide',
  content_css: '/tinymce/skins/content/default/content.css',
  promotion: false,
  branding: false,
  protect: [/<a.*?@click=.*?>/g],
  images_upload_handler: exampleImageUploadHandler,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  setup: (editor: any) => {
    // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access, @typescript-eslint/no-unsafe-call
    editor.ui.registry.addButton('copilot', {
      tooltip: 'Author copilot',
      icon: copilotIcon,
      onAction: () => {
        console.log('Copilot')
      }
    })
  }
})

const internalValue = ref(props.modelValue)

watch(internalValue, (newValue) => {
  emit('update:modelValue', newValue)
})

watch(() => props.modelValue, (newValue) => {
  internalValue.value = newValue
})

const handleEditorInput = (content: string) => {
  internalValue.value = content
}

onMounted(() => {
  initApiURL()
})
</script>
