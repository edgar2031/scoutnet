/**
 * Profile composable — load and save the user's freelance profile.
 *
 * Delegates raw HTTP calls to `useProfileApi`. Saving queues async
 * re-embedding on the backend; the composable returns immediately
 * and lets the backend process in the background.
 *
 * @example
 * const { profile, loading, save } = useProfile()
 */
import type { UpdateProfilePayload } from './apis/useProfileApi'

export function useProfile() {
  const profileApi = useProfileApi()

  const profile = ref({
    skills:     [] as string[],
    bio:        '',
    budget_min: 0,
    budget_max: 100000,
    language:   'en',
  })
  const loading          = ref(false)
  const embeddingQueued  = ref(false)

  /** Fetches the current user profile from the API. */
  async function fetchProfile(): Promise<void> {
    loading.value = true
    const res = await profileApi.get()
    Object.assign(profile.value, res)
    loading.value = false
  }

  /**
   * Persists partial profile fields and queues re-embedding.
   * Only provided fields are sent — omitted fields remain unchanged.
   *
   * @param updates - Partial profile fields to update
   * @returns True when the update was accepted by the server
   */
  async function save(updates: UpdateProfilePayload): Promise<boolean> {
    loading.value = true
    const res = await profileApi.update(updates)
    if (res.ok) Object.assign(profile.value, updates)
    embeddingQueued.value = res.embedding_queued
    loading.value = false
    return res.ok
  }

  onMounted(fetchProfile)

  return { profile, loading, embeddingQueued, save }
}
