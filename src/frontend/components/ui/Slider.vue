<script setup lang="ts">
/**
 * Touch-friendly range slider built on Swiper.
 *
 * Renders a single Swiper instance where each slide represents one step
 * increment. Swiping left/right updates the bound value. The thumb snaps
 * to the nearest step on release. Min/max/step labels are shown above the
 * track; the current value is highlighted in accent.
 *
 * @example
 * <Slider v-model="budget" :min="0" :max="10000" :step="500" unit="$" />
 */
import { Swiper, SwiperSlide } from 'swiper/vue'
import 'swiper/css'
import type { Swiper as SwiperInstance } from 'swiper'

const props = withDefaults(defineProps<{
  /** Minimum value. */
  min?: number
  /** Maximum value. */
  max?: number
  /** Step increment. */
  step?: number
  /** Unit prefix for value display (e.g. "$"). */
  unit?: string
}>(), { min: 0, max: 100, step: 1 })

const model = defineModel<number>()

// Total number of steps (= slides)
const steps = computed(() => Math.round((props.max - props.min) / props.step))

// Current slide index derived from model value
const slideIndex = computed(() =>
  Math.round(((model.value ?? props.min) - props.min) / props.step)
)

let sw: SwiperInstance | null = null

function onSwiper(instance: SwiperInstance) {
  sw = instance
  // Initialise position without transition
  sw.slideTo(slideIndex.value, 0)
}

function onSlideChange() {
  if (!sw) return
  model.value = props.min + sw.activeIndex * props.step
}

// Keep swiper in sync when model changes externally
watch(() => model.value, (val) => {
  if (!sw || val == null) return
  const idx = Math.round((val - props.min) / props.step)
  if (sw.activeIndex !== idx) sw.slideTo(idx, 150)
})
</script>

<template>
  <div class="w-full space-y-2 select-none">
    <!-- Labels row -->
    <div class="flex justify-between font-mono text-[10px] text-text-dim">
      <span>{{ unit }}{{ min }}</span>
      <span class="text-accent font-bold">{{ unit }}{{ model ?? min }}</span>
      <span>{{ unit }}{{ max }}</span>
    </div>

    <!-- Track wrapper -->
    <div class="relative h-6 flex items-center">
      <!-- Background track -->
      <div class="absolute inset-x-0 h-0.5 rounded-pill bg-border" />

      <!-- Filled track -->
      <div
        class="absolute left-0 h-0.5 rounded-pill bg-accent transition-all duration-150"
        :style="{ width: `${(slideIndex / steps) * 100}%` }"
      />

      <!-- Swiper handles touch dragging; slides are invisible — only the thumb matters -->
      <Swiper
        :slides-per-view="1"
        :initial-slide="slideIndex"
        :resistance="false"
        :speed="0"
        class="absolute inset-x-0 h-6 cursor-grab active:cursor-grabbing overflow-visible"
        @swiper="onSwiper"
        @slide-change="onSlideChange"
      >
        <SwiperSlide v-for="i in steps + 1" :key="i" class="w-full" />
      </Swiper>

      <!-- Thumb — positioned over the track by slideIndex -->
      <div
        class="absolute top-1/2 -translate-y-1/2 -translate-x-1/2 w-3.5 h-3.5 rounded-full bg-accent border-2 border-bg shadow-[0_0_6px_rgba(249,115,22,0.6)] pointer-events-none transition-all duration-150"
        :style="{ left: `${(slideIndex / steps) * 100}%` }"
      />
    </div>
  </div>
</template>
