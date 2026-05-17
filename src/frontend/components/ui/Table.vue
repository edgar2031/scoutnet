<script setup lang="ts">
/**
 * Generic data table with typed columns and row data.
 *
 * @example
 * <Table
 *   :columns="[{ key: 'date', label: 'Date' }, { key: 'cost', label: 'Cost' }]"
 *   :rows="usageRows"
 * />
 */
defineProps<{
  /** Column definitions — key maps to the row object property. */
  columns: { key: string; label: string; align?: 'left' | 'right' | 'center' }[]
  /** Row data — each row must be a plain object. */
  rows: Record<string, unknown>[]
  /** Shows a skeleton loading state instead of rows. */
  loading?: boolean
}>()
</script>

<template>
  <div class="w-full overflow-x-auto border border-border-subtle rounded-card overflow-hidden shadow-card">
    <table class="w-full text-sm">
      <thead>
        <tr class="border-b border-border bg-surface-2">
          <th
            v-for="col in columns"
            :key="col.key"
            :class="[
              'px-4 py-3 font-mono text-2xs uppercase tracking-widest text-text-dim font-normal',
              col.align === 'right'  && 'text-right',
              col.align === 'center' && 'text-center',
              (!col.align || col.align === 'left') && 'text-left',
            ]"
          >
            {{ col.label }}
          </th>
        </tr>
      </thead>
      <tbody>
        <!-- Loading skeleton -->
        <template v-if="loading">
          <tr v-for="i in 5" :key="i" class="border-b border-border">
            <td v-for="col in columns" :key="col.key" class="px-4 py-3">
              <div class="h-3 bg-surface-2 animate-pulse rounded-pill w-3/4" />
            </td>
          </tr>
        </template>

        <!-- Data rows -->
        <template v-else>
          <tr
            v-for="(row, i) in rows"
            :key="i"
            class="border-b border-border last:border-0 hover:bg-surface-2 transition-colors"
          >
            <td
              v-for="col in columns"
              :key="col.key"
              :class="[
                'px-4 py-3 font-mono text-xs text-text',
                col.align === 'right'  && 'text-right',
                col.align === 'center' && 'text-center',
              ]"
            >
              <slot :name="col.key" :value="row[col.key]" :row="row">
                {{ row[col.key] }}
              </slot>
            </td>
          </tr>
          <tr v-if="!rows.length">
            <td :colspan="columns.length" class="px-4 py-8 text-center font-mono text-xs text-text-dim">
              No data
            </td>
          </tr>
        </template>
      </tbody>
    </table>
  </div>
</template>
