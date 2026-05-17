<script setup lang="ts">
/**
 * 3D channel network graph — violet neon ring centre, rounded-square CSS2D icon nodes.
 *
 * Centre: dark sphere + three-layer violet torus rings (pulsing) + @you overlay.
 * Outer nodes: invisible raycasting sphere + CSS2D **rounded-square** icon box + label.
 * Source types: telegram (orange ✈), thread (violet #), upwork (yellow ▲), bot (red ⚡).
 *
 * @example
 * const ChannelGraph = defineAsyncComponent(() => import('~/components/graph/ChannelGraph.vue'))
 * <ChannelGraph @filter-channel="onFilter" />
 */
import * as THREE from 'three'
import { CSS2DRenderer, CSS2DObject } from 'three/addons/renderers/CSS2DRenderer.js'
import { Send, Briefcase, Zap, Hash } from 'lucide-vue-next'

const emit = defineEmits<{
  /** Emitted when user clicks a non-centre channel node. */
  filterChannel: [channelId: string, source: 'telegram' | 'upwork' | 'bot' | 'thread']
}>()

interface GraphNode {
  id:         string
  label:      string
  source:     'telegram' | 'upwork' | 'bot' | 'thread'
  leadsToday: number
  pos:        THREE.Vector3
}

const SOURCE_HEX: Record<GraphNode['source'], number> = {
  telegram: 0x4ade80,   // soft green — Cardinal TG style
  upwork:   0xe0a040,   // muted gold
  bot:      0xe06080,   // soft coral
  thread:   0x9580e0,   // soft lavender
}
const SOURCE_RGB: Record<GraphNode['source'], string> = {
  telegram: '74,222,128',
  upwork:   '224,160,64',
  bot:      '224,96,128',
  thread:   '149,128,224',
}
type SvgChild = [string, Record<string, string>]

/**
 * Extracts the SVG element descriptor array from a lucide-vue-next icon component.
 * Lucide components are render functions; calling them yields a VNode whose
 * props.iconNode holds the [tag, attrs][] array passed to createLucideIcon().
 */
function iconNode(lucideComponent: object): SvgChild[] {
  const vnode = (lucideComponent as Function)({}, { slots: {}, attrs: {} })
  return (vnode?.props?.iconNode ?? []) as SvgChild[]
}

const SOURCE_ICON: Record<GraphNode['source'], SvgChild[]> = {
  telegram: iconNode(Send),       // lucide: Send (paper plane — Telegram logo)
  upwork:   iconNode(Briefcase),  // lucide: Briefcase — freelance / Upwork
  bot:      iconNode(Zap),        // lucide: Zap — bot / automation
  thread:   iconNode(Hash),       // lucide: Hash — thread / forum channel
}
const SOURCE_TOOLTIP_CLASS: Record<GraphNode['source'], string> = {
  telegram: 'text-green-300   border-green-300/30   shadow-[0_0_12px_rgba(74,222,128,0.25)]',
  upwork:   'text-amber-300   border-amber-300/30   shadow-[0_0_12px_rgba(224,160,64,0.25)]',
  bot:      'text-rose-300    border-rose-300/30    shadow-[0_0_12px_rgba(224,96,128,0.25)]',
  thread:   'text-violet-300  border-violet-300/30  shadow-[0_0_12px_rgba(149,128,224,0.25)]',
}

// 18 nodes evenly distributed on a circle, r ≈ 2.8–3.7, Z ±0.25
// Positions scaled down 0.82× vs original to prevent CSS2D label clipping at container edges
const NODES: GraphNode[] = [
  { id: 'user',      label: '@you',           source: 'telegram', leadsToday: 0,  pos: new THREE.Vector3( 0.00,  0.00,  0.00) },
  // Telegram channels
  { id: 'tg-1',      label: '@FL_Hunter',      source: 'telegram', leadsToday: 14, pos: new THREE.Vector3( 3.36,  0.00,  0.20) },
  { id: 'tg-2',      label: '@VuejsJobs',      source: 'telegram', leadsToday: 9,  pos: new THREE.Vector3( 3.03,  1.60, -0.15) },
  { id: 'tg-3',      label: '@Dev_Market',     source: 'telegram', leadsToday: 6,  pos: new THREE.Vector3( 1.80,  2.95,  0.20) },
  { id: 'tg-4',      label: '@Stack_Jobs',     source: 'telegram', leadsToday: 4,  pos: new THREE.Vector3( 0.16,  3.36, -0.20) },
  { id: 'tg-5',      label: '@gDev_Digest',    source: 'telegram', leadsToday: 11, pos: new THREE.Vector3(-1.60,  3.03,  0.25) },
  { id: 'tg-6',      label: '@Outsors_Chat',   source: 'telegram', leadsToday: 7,  pos: new THREE.Vector3(-2.95,  1.80, -0.20) },
  { id: 'tg-7',      label: '@Startup_Jobs',   source: 'telegram', leadsToday: 5,  pos: new THREE.Vector3(-3.36,  0.00,  0.20) },
  { id: 'tg-8',      label: '@gFL_Network',    source: 'telegram', leadsToday: 8,  pos: new THREE.Vector3(-3.03, -1.60, -0.15) },
  { id: 'tg-9',      label: '@FullStack_WS',   source: 'telegram', leadsToday: 3,  pos: new THREE.Vector3(-1.80, -2.95,  0.25) },
  { id: 'tg-10',     label: '@Next_Orders',    source: 'telegram', leadsToday: 6,  pos: new THREE.Vector3(-0.16, -3.36, -0.20) },
  // Thread channels
  { id: 'th-1',      label: '@gDev_Telega',    source: 'thread',   leadsToday: 7,  pos: new THREE.Vector3( 2.79, -2.05,  0.15) },
  { id: 'th-2',      label: '@Lead_Hunter',    source: 'thread',   leadsToday: 4,  pos: new THREE.Vector3( 3.69, -0.66, -0.20) },
  // Upwork
  { id: 'uw-1',      label: '@Upwork_Feed',    source: 'upwork',   leadsToday: 9,  pos: new THREE.Vector3( 1.60, -3.03,  0.20) },
  { id: 'uw-2',      label: '@Kadrop_Bot',     source: 'upwork',   leadsToday: 4,  pos: new THREE.Vector3( 0.66, -3.69, -0.15) },
  // Bot channels
  { id: 'bot-1',     label: '@TG_Freelance',   source: 'bot',      leadsToday: 5,  pos: new THREE.Vector3(-1.07, -3.53,  0.20) },
  { id: 'bot-2',     label: '@Crypto_Dev',     source: 'bot',      leadsToday: 2,  pos: new THREE.Vector3(-2.30, -2.71, -0.20) },
  { id: 'bot-3',     label: '@OpenSea_WS',     source: 'bot',      leadsToday: 3,  pos: new THREE.Vector3( 2.30,  2.71,  0.10) },
  { id: 'bot-4',     label: '@Team_Leads',     source: 'bot',      leadsToday: 6,  pos: new THREE.Vector3(-0.66,  3.69, -0.25) },
]

const containerRef = ref<HTMLDivElement | null>(null)
const hoveredId    = ref<string | null>(null)
const hoveredNode  = computed(() => NODES.find(n => n.id === hoveredId.value) ?? null)

let renderer:     THREE.WebGLRenderer
let labelRenderer: CSS2DRenderer
let scene:        THREE.Scene
let camera:       THREE.PerspectiveCamera
let group:        THREE.Group
let ringMeshes:   THREE.Mesh[]  = []
let hitMeshes:    THREE.Mesh[]  = []
let iconEls:      HTMLElement[] = []
let hitNodes:     GraphNode[]   = []
let animId:       number
let raycaster:    THREE.Raycaster
let mouse:        THREE.Vector2
let clock:        THREE.Clock

/** Builds a DOM SVG element from a Lucide icon descriptor array. */
function makeLucideSvg(children: [string, Record<string, string>][], color: string, size: number): SVGSVGElement {
  const NS = 'http://www.w3.org/2000/svg'
  const svg = document.createElementNS(NS, 'svg')
  svg.setAttribute('viewBox', '0 0 24 24')
  svg.setAttribute('width',  String(size))
  svg.setAttribute('height', String(size))
  svg.setAttribute('fill',   'none')
  svg.setAttribute('stroke', color)
  svg.setAttribute('stroke-width',    '2')
  svg.setAttribute('stroke-linecap',  'round')
  svg.setAttribute('stroke-linejoin', 'round')
  svg.style.display = 'block'
  for (const [tag, attrs] of children) {
    const el = document.createElementNS(NS, tag)
    for (const [k, v] of Object.entries(attrs)) el.setAttribute(k, v)
    svg.appendChild(el)
  }
  return svg
}

// ─── CSS2D node icon (rounded square — matches reference) ────
function makeIconEl(node: GraphNode): HTMLElement {
  const rgb  = SOURCE_RGB[node.source]
  const sz   = Math.round(34 + Math.min(node.leadsToday, 14) * 0.6)  // 34–43px
  const icSz = Math.round(sz * 0.52)
  const box  = document.createElement('div')
  box.style.cssText = `
    width:${sz}px;height:${sz}px;
    border-radius:9px;
    background:rgba(5,7,15,0.92);
    border:1.5px solid rgba(${rgb},0.65);
    box-shadow:0 0 12px rgba(${rgb},0.45),inset 0 0 8px rgba(${rgb},0.08);
    display:flex;align-items:center;justify-content:center;
    transition:box-shadow 0.18s,border-color 0.18s;
    pointer-events:none;
  `
  box.appendChild(makeLucideSvg(SOURCE_ICON[node.source], `rgba(${rgb},0.95)`, icSz))
  return box
}

function makeLabelEl(text: string): HTMLElement {
  const span = document.createElement('span')
  span.textContent = text
  span.style.cssText = `
    font-family:'JetBrains Mono','Fira Mono',monospace;
    font-size:8px;letter-spacing:0.06em;
    color:rgba(160,180,210,0.75);
    white-space:nowrap;pointer-events:none;
    text-transform:uppercase;
  `
  return span
}

// ─── scene ──────────────────────────────────────────────────
function buildScene(container: HTMLDivElement) {
  const w = container.clientWidth
  const h = container.clientHeight

  renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true })
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2))
  renderer.setSize(w, h)
  renderer.setClearColor(0x000000, 0)
  container.appendChild(renderer.domElement)

  labelRenderer = new CSS2DRenderer()
  labelRenderer.setSize(w, h)
  labelRenderer.domElement.style.cssText = 'position:absolute;top:0;left:0;pointer-events:none;'
  container.appendChild(labelRenderer.domElement)

  scene  = new THREE.Scene()
  camera = new THREE.PerspectiveCamera(50, w / h, 0.1, 100)
  camera.position.set(0, 0, 13)

  clock     = new THREE.Clock()
  raycaster = new THREE.Raycaster()
  mouse     = new THREE.Vector2(9999, 9999)

  scene.add(new THREE.AmbientLight(0xffffff, 0.18))
  const dir = new THREE.DirectionalLight(0xffffff, 0.40)
  dir.position.set(5, 8, 5)
  scene.add(dir)

  // Violet point light — ring atmosphere
  const pV = new THREE.PointLight(0x8b5cf6, 2.2, 14)
  pV.position.set(0, 0, 1)
  scene.add(pV)

  // Faint cool blue fill — Cardinal-style atmosphere
  const pO = new THREE.PointLight(0x3b82f6, 0.15, 22)
  pO.position.set(5, 3, 2)
  scene.add(pO)

  // Stars — visible particle field matching Cardinal reference
  const STAR = 1400
  const sp   = new Float32Array(STAR * 3)
  for (let i = 0; i < STAR * 3; i++) sp[i] = (Math.random() - 0.5) * 50
  const sg   = new THREE.BufferGeometry()
  sg.setAttribute('position', new THREE.BufferAttribute(sp, 3))
  scene.add(new THREE.Points(sg, new THREE.PointsMaterial({ color: 0x4a5580, size: 0.06, transparent: true, opacity: 0.7 })))

  group = new THREE.Group()
  scene.add(group)

  // Edges
  const uPos = NODES[0].pos
  for (const node of NODES.slice(1)) {
    const geo = new THREE.BufferGeometry().setFromPoints([uPos, node.pos])
    group.add(new THREE.Line(geo, new THREE.LineBasicMaterial({
      color: SOURCE_HEX[node.source], transparent: true, opacity: 0.22,
    })))
  }

  // Centre dark sphere
  const cS = new THREE.Mesh(
    new THREE.SphereGeometry(0.70, 48, 48),
    new THREE.MeshStandardMaterial({ color: 0x040610, roughness: 0.95, metalness: 0 }),
  )
  cS.userData = { id: 'user' }
  group.add(cS)

  // Centre CSS2D — @you text
  const cW  = document.createElement('div')
  cW.style.cssText = 'display:flex;flex-direction:column;align-items:center;gap:2px;pointer-events:none;'
  const cN  = document.createElement('span')
  cN.textContent  = '@you'
  cN.style.cssText = `
    font-family:'JetBrains Mono',monospace;font-size:15px;font-weight:700;
    color:rgba(232,236,245,0.96);white-space:nowrap;letter-spacing:0.04em;
  `
  const cSt = document.createElement('span')
  cSt.textContent  = `${NODES.length - 1} channels`
  cSt.style.cssText = `
    font-family:'JetBrains Mono',monospace;font-size:9px;letter-spacing:0.10em;
    color:rgba(139,92,246,0.90);white-space:nowrap;text-transform:uppercase;
  `
  cW.appendChild(cN)
  cW.appendChild(cSt)
  const cL = new CSS2DObject(cW)
  cL.position.set(0, 0, 0)
  cS.add(cL)

  // Violet neon rings
  const V = 0x8b5cf6

  // Core — thin, very bright
  const r1 = new THREE.Mesh(
    new THREE.TorusGeometry(1.35, 0.030, 24, 150),
    new THREE.MeshStandardMaterial({ color: V, emissive: new THREE.Color(V), emissiveIntensity: 3.8, roughness: 0, metalness: 0 }),
  )
  group.add(r1); ringMeshes.push(r1)

  // Mid bloom
  const r2 = new THREE.Mesh(
    new THREE.TorusGeometry(1.35, 0.18, 10, 150),
    new THREE.MeshStandardMaterial({ color: V, emissive: new THREE.Color(V), emissiveIntensity: 1.1, transparent: true, opacity: 0.28, roughness: 0 }),
  )
  group.add(r2); ringMeshes.push(r2)

  // Outer halo
  const r3 = new THREE.Mesh(
    new THREE.TorusGeometry(1.72, 0.018, 8, 110),
    new THREE.MeshStandardMaterial({ color: V, emissive: new THREE.Color(V), emissiveIntensity: 0.75, transparent: true, opacity: 0.16, roughness: 0 }),
  )
  group.add(r3); ringMeshes.push(r3)

  // Outer nodes
  for (const node of NODES.slice(1)) {
    const hit = new THREE.Mesh(
      new THREE.SphereGeometry(0.52, 6, 6),
      new THREE.MeshBasicMaterial({ transparent: true, opacity: 0 }),
    )
    hit.position.copy(node.pos)
    hit.userData = { id: node.id }
    group.add(hit)
    hitMeshes.push(hit)
    hitNodes.push(node)

    const iconEl = makeIconEl(node)
    iconEls.push(iconEl)
    const iconObj = new CSS2DObject(iconEl)
    iconObj.position.set(0, 0, 0)
    hit.add(iconObj)

    const lblObj = new CSS2DObject(makeLabelEl(node.label))
    lblObj.position.set(0, -0.62, 0)
    hit.add(lblObj)
  }
}

// ─── animation ──────────────────────────────────────────────
function animate() {
  animId = requestAnimationFrame(animate)
  const t = clock.getElapsedTime()

  group.rotation.z += 0.0018   // Z-axis keeps the ring circular (not oval) when viewed from camera

  const p  = Math.sin(t * 1.85) * 0.5 + 0.5
  const sc = 1 + p * 0.020
  ;(ringMeshes[0].material as THREE.MeshStandardMaterial).emissiveIntensity = 3.2 + p * 1.5
  ;(ringMeshes[1].material as THREE.MeshStandardMaterial).emissiveIntensity = 0.80 + p * 0.55
  ringMeshes.forEach(r => r.scale.setScalar(sc))

  raycaster.setFromCamera(mouse, camera)
  const hits  = raycaster.intersectObjects(hitMeshes)
  const hitId = (hits[0]?.object?.userData?.id as string) ?? null
  hoveredId.value = hitId

  for (let i = 0; i < hitMeshes.length; i++) {
    const node  = hitNodes[i]
    const icon  = iconEls[i]
    const hover = node.id === hitId
    const rgb   = SOURCE_RGB[node.source]
    icon.style.boxShadow  = hover
      ? `0 0 20px rgba(${rgb},1.0),inset 0 0 10px rgba(${rgb},0.22)`
      : `0 0 12px rgba(${rgb},0.45),inset 0 0 8px rgba(${rgb},0.08)`
    icon.style.borderColor = `rgba(${rgb},${hover ? '1' : '0.65'})`
  }

  renderer.render(scene, camera)
  labelRenderer.render(scene, camera)
}

function onMouseMove(e: MouseEvent) {
  const rect = containerRef.value!.getBoundingClientRect()
  mouse.x =  ((e.clientX - rect.left) / rect.width)  * 2 - 1
  mouse.y = -((e.clientY - rect.top)  / rect.height) * 2 + 1
}
function onMouseLeave() { mouse.set(9999, 9999); hoveredId.value = null }
function onClick() {
  if (!hoveredId.value) return
  const node = NODES.find(n => n.id === hoveredId.value)
  if (node) emit('filterChannel', node.id, node.source)
}
function onResize() {
  if (!containerRef.value) return
  const w = containerRef.value.clientWidth
  const h = containerRef.value.clientHeight
  camera.aspect = w / h
  camera.updateProjectionMatrix()
  renderer.setSize(w, h)
  labelRenderer.setSize(w, h)
}

onMounted(() => { if (!containerRef.value) return; buildScene(containerRef.value); animate(); window.addEventListener('resize', onResize) })
onBeforeUnmount(() => { cancelAnimationFrame(animId); renderer?.dispose(); window.removeEventListener('resize', onResize) })
</script>

<template>
  <div
    ref="containerRef"
    class="relative w-full h-full min-h-[300px] overflow-hidden"
    style="background:#060a15"
    :class="hoveredId ? 'cursor-pointer' : 'cursor-default'"
    @mousemove="onMouseMove"
    @mouseleave="onMouseLeave"
    @click="onClick"
  >
    <Transition name="fade">
      <div
        v-if="hoveredNode"
        class="absolute top-4 left-1/2 -translate-x-1/2 pointer-events-none z-20
               font-mono text-[10px] tracking-widest uppercase
               px-3 py-1.5 border rounded-pill bg-[rgba(4,6,16,0.95)]"
        :class="SOURCE_TOOLTIP_CLASS[hoveredNode.source]"
      >
        {{ hoveredNode.label }} · {{ hoveredNode.leadsToday }} leads today
      </div>
    </Transition>

    <!-- Legend -->
    <div class="absolute bottom-3 left-3 z-10 flex gap-3 font-mono text-[10px] pointer-events-none items-center">
      <span class="flex items-center gap-1 text-green-300"><Send :size="10" /> TG</span>
      <span class="flex items-center gap-1 text-violet-300"><Hash :size="10" /> Thread</span>
      <span class="flex items-center gap-1 text-amber-300"><Briefcase :size="10" /> Upwork</span>
      <span class="flex items-center gap-1 text-rose-300"><Zap :size="10" /> Bot</span>
    </div>
  </div>
</template>

<style scoped>
.fade-enter-active, .fade-leave-active { transition: opacity 0.15s ease; }
.fade-enter-from,  .fade-leave-to      { opacity: 0; }
</style>
