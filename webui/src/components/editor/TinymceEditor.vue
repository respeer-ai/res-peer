<template>
  <div>
    <editor
      :init='editorInit'
      @input='handleEditorInput'
      v-model='internalValue'
      @selection-change='handleSelectionChange'
    />
  </div>
  <q-dialog v-model='showing' full-width>
    <div :style='{padding: "96px"}' v-if='taskType !== TaskType.GenerateIllustrate'>
      <TextCopilot :text='selectedText' :task-type='taskType' @cancel='onCopilotCancel' @change-text='(text) => onChangeText(text)' />
    </div>
    <div :style='{padding: "96px"}' v-else>
      <IllustrateCopilot :text='selectedText' @cancel='onCopilotCancel' @generated='(base64) => onCoverGenerated(base64)' />
    </div>
  </q-dialog>
</template>

<script setup lang='ts'>
/* eslint-disable  @typescript-eslint/no-unsafe-call, @typescript-eslint/no-unsafe-member-access, @typescript-eslint/no-explicit-any, @typescript-eslint/no-unsafe-assignment */
import { ref, watch, defineProps, withDefaults, defineEmits, onMounted } from 'vue'
import Editor from '@tinymce/tinymce-vue'
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
import { TaskType, taskTypeName } from 'src/stores/cpregistry'

import TextCopilot from './TextCopilot.vue'
import IllustrateCopilot from './IllustrateCopilot.vue'

const apiURL = ref('')

const showing = ref(false)
const taskType = ref(TaskType.FixGrammar)

// eslint-disable-next-line @typescript-eslint/no-unused-vars
const uploadMedia = (resolve: (value: string) => void, reject: (value: string) => void, filename: string, base64: string) => {
  // TODO
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
          'forecolor backcolor bold italic underline strikethrough link anchor |' +
          'customToolBar |' +
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
  menubar: 'edit view insert format tools table custom',
  menu: {
    custom: {
      title: 'Copilot',
      items: 'fixGrammar RewriteEasierUnderstand Paraphrase WriteFormally WriteMoreNeutral GenerateIllustrate'
    }
  },
  plugins: props.plugins,
  toolbar: props.toolbar,
  skin_url: '/tinymce/skins/ui/oxide',
  content_css: '/tinymce/skins/content/default/content.css',
  promotion: false,
  branding: false,
  protect: [/<a.*?@click=.*?>/g],
  images_upload_handler: exampleImageUploadHandler,
  setup: (editor: any) => {
    const svgContent = `
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
      <g clip-path="url(#clip0_564_185)">
      <path d="M2.4536 13.1206L3.52602 14.1931C3.86468 14.5317 4.34445 14.7293 4.796 14.7293C5.24754 14.7293 5.72731 14.5317 6.06597 14.1931L12.7168 7.54227L13.0554 4.32497C13.1119 3.92987 12.8014 3.5912 12.4063 3.5912C12.3781 3.5912 12.3499 3.5912 12.3217 3.5912L9.10438 3.92987L2.4536 10.5807C1.74806 11.2862 1.74806 12.4151 2.4536 13.1206ZM9.72526 5.22807L11.6161 5.03052L11.4186 6.92139L8.21082 10.1573L7.33595 11.0322L5.61443 9.31067L6.4893 8.43579L9.72526 5.22807ZM3.41314 11.5402L4.65489 10.2702L6.37641 11.9917L5.10644 13.2335C4.99355 13.3464 4.85244 13.3746 4.796 13.3746C4.73955 13.3746 4.59845 13.3464 4.48556 13.2335L3.41314 12.1611C3.24381 11.9917 3.24381 11.7095 3.41314 11.5402Z" fill="#1A1A1A" fill-opacity="0.6"/>
      <path d="M2.89809 1.00881C3.00872 0.552564 3.65762 0.552563 3.76825 1.00881L4.07267 2.26422C4.11217 2.42711 4.23935 2.5543 4.40225 2.5938L5.65765 2.89821C6.1139 3.00885 6.1139 3.65774 5.65765 3.76837L4.40225 4.07279C4.23935 4.11229 4.11217 4.23947 4.07267 4.40237L3.76825 5.65777C3.65762 6.11402 3.00872 6.11402 2.89809 5.65777L2.59367 4.40237C2.55417 4.23947 2.42699 4.11229 2.26409 4.07279L1.00869 3.76837C0.552441 3.65774 0.552441 3.00885 1.00869 2.89821L2.26409 2.5938C2.42699 2.5543 2.55417 2.42711 2.59367 2.26422L2.89809 1.00881Z" fill="#1A1A1A" fill-opacity="0.6"/>
      <path fill-rule="evenodd" clip-rule="evenodd" d="M3.33317 1.49426L3.11577 2.39082C3.02887 2.74919 2.74906 3.02899 2.3907 3.11589L1.49414 3.33329L2.3907 3.55069C2.74906 3.63759 3.02887 3.9174 3.11577 4.27577L3.33317 5.17232L3.55057 4.27577C3.63747 3.9174 3.91728 3.63759 4.27565 3.55069L5.1722 3.33329L4.27565 3.11589C3.91728 3.02899 3.63747 2.74919 3.55057 2.39082L3.33317 1.49426ZM3.76825 1.00881C3.65762 0.552563 3.00872 0.552564 2.89809 1.00881L2.59367 2.26422C2.55417 2.42711 2.42699 2.5543 2.26409 2.5938L1.00869 2.89821C0.552441 3.00885 0.552441 3.65774 1.00869 3.76837L2.26409 4.07279C2.42699 4.11229 2.55417 4.23947 2.59367 4.40237L2.89809 5.65777C3.00872 6.11402 3.65762 6.11402 3.76825 5.65777L4.07267 4.40237C4.11217 4.23947 4.23935 4.11229 4.40225 4.07279L5.65765 3.76837C6.1139 3.65774 6.1139 3.00885 5.65765 2.89821L4.40225 2.5938C4.23935 2.5543 4.11217 2.42711 4.07267 2.26422L3.76825 1.00881Z" fill="#1A1A1A" fill-opacity="0.6"/>
      <path d="M13.0069 11.5899C13.0898 11.2477 13.5765 11.2477 13.6595 11.5899L13.8878 12.5315C13.9174 12.6537 14.0128 12.749 14.135 12.7787L15.0765 13.007C15.4187 13.09 15.4187 13.5766 15.0765 13.6596L14.135 13.8879C14.0128 13.9175 13.9174 14.0129 13.8878 14.1351L13.6595 15.0767C13.5765 15.4188 13.0898 15.4188 13.0069 15.0767L12.7785 14.1351C12.7489 14.0129 12.6535 13.9175 12.5314 13.8879L11.5898 13.6596C11.2476 13.5766 11.2476 13.09 11.5898 13.007L12.5314 12.7787C12.6535 12.749 12.7489 12.6537 12.7785 12.5315L13.0069 11.5899Z" fill="#1A1A1A" fill-opacity="0.6"/>
      <path fill-rule="evenodd" clip-rule="evenodd" d="M13.3332 11.954L13.1701 12.6264C13.1049 12.8952 12.8951 13.1051 12.6263 13.1702L11.9539 13.3333L12.6263 13.4963C12.8951 13.5615 13.1049 13.7714 13.1701 14.0401L13.3332 14.7126L13.4962 14.0401C13.5614 13.7714 13.7713 13.5615 14.04 13.4963L14.7124 13.3333L14.04 13.1702C13.7713 13.1051 13.5614 12.8952 13.4962 12.6264L13.3332 11.954ZM13.6595 11.5899C13.5765 11.2477 13.0898 11.2477 13.0069 11.5899L12.7785 12.5315C12.7489 12.6537 12.6535 12.749 12.5314 12.7787L11.5898 13.007C11.2476 13.09 11.2476 13.5766 11.5898 13.6596L12.5314 13.8879C12.6535 13.9175 12.7489 14.0129 12.7785 14.1351L13.0069 15.0767C13.0898 15.4188 13.5765 15.4188 13.6595 15.0767L13.8878 14.1351C13.9174 14.0129 14.0128 13.9175 14.135 13.8879L15.0765 13.6596C15.4187 13.5766 15.4187 13.09 15.0765 13.007L14.135 12.7787C14.0128 12.749 13.9174 12.6537 13.8878 12.5315L13.6595 11.5899Z" fill="#1A1A1A" fill-opacity="0.6"/>
      <path d="M15.3332 1.33329C15.3332 1.70148 15.0347 1.99996 14.6665 1.99996C14.2983 1.99996 13.9998 1.70148 13.9998 1.33329C13.9998 0.965103 14.2983 0.666626 14.6665 0.666626C15.0347 0.666626 15.3332 0.965103 15.3332 1.33329Z" fill="#1A1A1A" fill-opacity="0.6"/>
      <path d="M8.6665 1.33329C8.6665 1.70148 8.36803 1.99996 7.99984 1.99996C7.63165 1.99996 7.33317 1.70148 7.33317 1.33329C7.33317 0.965103 7.63165 0.666626 7.99984 0.666626C8.36803 0.666626 8.6665 0.965103 8.6665 1.33329Z" fill="#1A1A1A" fill-opacity="0.6"/>
      <path d="M2.6665 7.99996C2.6665 8.36815 2.36803 8.66663 1.99984 8.66663C1.63165 8.66663 1.33317 8.36815 1.33317 7.99996C1.33317 7.63177 1.63165 7.33329 1.99984 7.33329C2.36803 7.33329 2.6665 7.63177 2.6665 7.99996Z" fill="#1A1A1A" fill-opacity="0.6"/>
      </g>
      <defs>
      <clipPath id="clip0_564_185">
      <rect width="16" height="16" fill="white"/>
      </clipPath>
      </defs>
    </svg>`
    // Register the custom icon
    editor.ui.registry.addIcon('copilotIcon', svgContent)

    /* adding a menu button */
    editor.ui.registry.addMenuItem('fixGrammar', {
      text: taskTypeName(TaskType.FixGrammar),
      onAction: function () {
        onParagraphCopilot(TaskType.FixGrammar)
      }
    })
    editor.ui.registry.addMenuItem('RewriteEasierUnderstand', {
      text: taskTypeName(TaskType.RewriteEasierUnderstand),
      onAction: function () {
        onParagraphCopilot(TaskType.RewriteEasierUnderstand)
      }
    })
    editor.ui.registry.addMenuItem('Paraphrase', {
      text: taskTypeName(TaskType.Paraphrase),
      onAction: function () {
        onParagraphCopilot(TaskType.Paraphrase)
      }
    })
    editor.ui.registry.addMenuItem('WriteFormally', {
      text: taskTypeName(TaskType.WriteFormally),
      onAction: function () {
        onParagraphCopilot(TaskType.WriteFormally)
      }
    })
    editor.ui.registry.addMenuItem('WriteMoreNeutral', {
      text: taskTypeName(TaskType.WriteMoreNeutral),
      onAction: function () {
        onParagraphCopilot(TaskType.WriteMoreNeutral)
      }
    })
    editor.ui.registry.addMenuItem('GenerateIllustrate', {
      text: taskTypeName(TaskType.GenerateIllustrate),
      onAction: function () {
        onParagraphCopilot(TaskType.GenerateIllustrate)
      }
    })
    /* adding a toolbar menu button */
    editor.ui.registry.addMenuButton('customToolBar', {
      text: '',
      icon: 'copilotIcon', // use custom icon
      fetch: function (callback: (arg0: { type: string; text: string; onAction: () => void }[]) => void) {
        const items = [
          {
            type: 'menuitem',
            text: taskTypeName(TaskType.FixGrammar),
            onAction: function () {
              onParagraphCopilot(TaskType.FixGrammar)
            }
          },
          {
            type: 'menuitem',
            text: taskTypeName(TaskType.RewriteEasierUnderstand),
            onAction: function () {
              onParagraphCopilot(TaskType.RewriteEasierUnderstand)
            }
          },
          {
            type: 'menuitem',
            text: taskTypeName(TaskType.Paraphrase),
            onAction: function () {
              onParagraphCopilot(TaskType.Paraphrase)
            }
          },
          {
            type: 'menuitem',
            text: taskTypeName(TaskType.WriteFormally),
            onAction: function () {
              onParagraphCopilot(TaskType.WriteFormally)
            }
          },
          {
            type: 'menuitem',
            text: taskTypeName(TaskType.WriteMoreNeutral),
            onAction: function () {
              onParagraphCopilot(TaskType.WriteMoreNeutral)
            }
          },
          {
            type: 'menuitem',
            text: taskTypeName(TaskType.GenerateIllustrate),
            onAction: function () {
              onParagraphCopilot(TaskType.GenerateIllustrate)
            }
          }
        ]
        callback(items)
      }
    })
  }
})

const internalValue = ref(props.modelValue)

const selectedText = ref('')
const startOffset = ref(0)
const endOffset = ref(0)
const selectionNode = ref(undefined as any)

const handleSelectionChange = (event: any, editor: any) => {
  selectedText.value = editor.selection.getContent({ format: 'text' })
  const range = editor.selection.getRng()
  startOffset.value = range.startOffset
  endOffset.value = range.endOffset
  selectionNode.value = editor.selection
}

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

const onParagraphCopilot = (_taskType: TaskType) => {
  if (!selectedText.value.length) return
  showing.value = true
  taskType.value = _taskType
}

const onCopilotCancel = () => {
  showing.value = false
}

const onChangeText = (text: string) => {
  selectionNode.value?.setContent(text)
  // TODO： hight result text
  showing.value = false
}

const onCoverGenerated = (base64: string) => {
  const text = `${selectedText.value}<br><br><img width="720px" fit="contain" src="${base64}"><br><br>`
  selectionNode.value?.setContent(text)
  showing.value = false
}

</script>
