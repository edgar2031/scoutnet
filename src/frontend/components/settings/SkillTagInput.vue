<script setup lang="ts">
/**
 * Tag chip input for skill selection.
 *
 * Press Enter or comma to add a tag. Click × on a chip to remove it.
 * Emits `update:modelValue` on every change.
 *
 * @example
 * <SkillTagInput v-model="skills" placeholder="Add skill..." />
 */
import { X } from 'lucide-vue-next'

const props = defineProps<{
  /** Current list of skill tags. */
  modelValue: string[]
  /** Input placeholder text. */
  placeholder?: string
}>()

const emit = defineEmits<{
  /**
   * Fired when the skills list changes.
   * @param skills Updated skills array.
   */
  'update:modelValue': [skills: string[]]
}>()

const input = ref('')

/**
 * Adds a new tag from the current input value.
 * No-ops if the input is empty or the tag already exists.
 */
function addTag() {
  const tag = input.value.trim().replace(/,$/, '')
  if (tag && !props.modelValue.includes(tag)) {
    emit('update:modelValue', [...props.modelValue, tag])
  }
  input.value = ''
}

/**
 * Removes a tag by value.
 * @param tag - The tag string to remove.
 */
function removeTag(tag: string) {
  emit('update:modelValue', props.modelValue.filter(t => t !== tag))
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter' || e.key === ',') { e.preventDefault(); addTag() }
  if (e.key === 'Backspace' && !input.value && props.modelValue.length) {
    emit('update:modelValue', props.modelValue.slice(0, -1))
  }
}
</script>

<template>
  <div class="flex flex-wrap gap-1.5 bg-surface-2 border border-border rounded-xl p-2.5 focus-within:border-accent transition-colors">
    <span
      v-for="tag in modelValue"
      :key="tag"
      class="flex items-center gap-1 font-mono text-xs bg-surface border border-border rounded-lg text-text px-2.5 py-1"
    >
      {{ tag }}
      <button type="button" class="text-text-dim hover:text-accent-2 transition-colors leading-none" aria-label="Remove" @click="removeTag(tag)">
        <X :size="10" :stroke-width="2.5" />
      </button>
    </span>
    <input
      v-model="input"
      :placeholder="placeholder ?? 'Add skill…'"
      class="flex-1 min-w-[120px] bg-transparent font-mono text-sm text-text placeholder:text-text-dim outline-none"
      @keydown="onKeydown"
      @blur="addTag"
    />
  </div>
</template>
