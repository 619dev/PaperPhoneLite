import { allLangs, t, type LangCode, type TranslationParams } from './index'

/** Non-React crypto/network code reads the active locale when a message is
 * produced; importing this module never freezes the language or loads the store. */
export function securityText(key: string, params?: TranslationParams): string {
  let lang: LangCode = 'zh'
  try {
    const current = localStorage.getItem('lang') as LangCode
    if (allLangs.includes(current)) lang = current
  } catch { /* Use the default before browser storage is available. */ }
  return t(key, lang, params)
}

const API_ERRORS: Record<string, string> = {
  'Group operation failed': 'grouptrust.operation_failed',
  'Not a group member': 'grouptrust.not_member',
  'Group not found': 'grouptrust.not_found',
  'Group epoch exhausted': 'grouptrust.epoch_exhausted',
  'Group member limit exceeded': 'grouptrust.member_limit',
  'Only group administrators can edit the group': 'grouptrust.admin_edit',
  'Only group administrators can remove members': 'grouptrust.admin_remove',
  'Member not found': 'grouptrust.member_not_found',
  'Cannot remove group owner or peer administrator': 'grouptrust.protected_member',
  'Group encryption is required': 'grouptrust.encryption_required',
  'Sender key epoch expired; refresh group membership': 'grouptrust.epoch_expired',
  'Signed key distributions must cover exactly the current members, including self': 'grouptrust.recipients',
  'Sender key already published; fetch existing key': 'grouptrust.already_published',
  'Invalid or expired invite': 'grouptrust.invalid_invite',
  'Only owner can disband': 'grouptrust.owner_disband',
  'Only the group owner can toggle encryption': 'group.encryption_owner_only',
}

export function localizeSecurityApiError(message: string): string {
  return API_ERRORS[message] ? securityText(API_ERRORS[message]) : message
}
