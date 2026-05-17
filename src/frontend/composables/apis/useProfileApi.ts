/**
 * Raw API calls for the user profile resource — fetch and update.
 *
 * @example
 * const profileApi = useProfileApi()
 * const profile = await profileApi.get()
 * await profileApi.update({ skills: ['Vue 3', 'Rust'] })
 */

export interface Profile {
  /** Ordered skill tags used for similarity matching. */
  skills:     string[]
  /** Short bio embedded alongside skills for semantic search. */
  bio:        string
  /** Minimum acceptable project budget in the user's currency. */
  budget_min: number
  /** Maximum acceptable project budget. */
  budget_max: number
  /** ISO 639-1 language code for preferred AI response language. */
  language:   string
}

export interface UpdateProfilePayload {
  skills?:     string[]
  bio?:        string
  budget_min?: number
  budget_max?: number
  language?:   string
}

export interface UpdateProfileResponse {
  /** True when profile was saved and re-embedding was queued. */
  ok:               boolean
  /** Server-side timestamp of the update. */
  updated_at:       string
  /** True when the embedding job was queued (false if unchanged). */
  embedding_queued: boolean
}

export function useProfileApi() {
  const api = useApi()

  /**
   * Fetches the current user's profile.
   *
   * @returns Full profile object
   * @throws `ApiError` — 401 when unauthenticated
   */
  function get(): Promise<Profile> {
    return api.get<Profile>('/profile')
  }

  /**
   * Partially updates the profile and queues re-embedding.
   * Only the provided fields are updated; omitted fields are unchanged.
   *
   * @param payload - Partial profile update
   * @returns Update result including embedding queue status
   * @throws `ApiError` — 422 on validation failure (e.g. budget_min > budget_max)
   */
  function update(payload: UpdateProfilePayload): Promise<UpdateProfileResponse> {
    return api.patch<UpdateProfileResponse>('/profile', payload)
  }

  return { get, update }
}
