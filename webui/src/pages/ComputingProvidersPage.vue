<template>
  <div class='row'>
    <q-space />
    <div :style='{width:"960px"}'>
      <CPNodeCard
        v-for='(cpNode, i) in cpNodes'
        :key='cpNode.nodeId'
        :cp-node='cpNode'
        :style='{margin:"64px 0",paddingBottom:"32px",borderBottom: i < cpNodes.length - 1 ? "1px solid grey" : ""}'
      />
    </div>
    <q-space />
  </div>
</template>

<script lang='ts' setup>
import { computed } from 'vue'
import { useCPRegistryStore } from 'src/stores/cpregistry'

import CPNodeCard from 'src/components/CPNodeCard.vue'

const cpRegistry = useCPRegistryStore()
const cpNodes = computed(() => cpRegistry.nodes)

</script>
