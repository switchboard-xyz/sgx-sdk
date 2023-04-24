import * as VerificationStatus from './VerificationStatus';
import * as SwitchboardPermission from './SwitchboardPermission';

export { NodeHeartbeatParams } from './NodeHeartbeatParams';
export type {
  NodeHeartbeatParamsFields,
  NodeHeartbeatParamsJSON,
} from './NodeHeartbeatParams';
export { NodeInitParams } from './NodeInitParams';
export type {
  NodeInitParamsFields,
  NodeInitParamsJSON,
} from './NodeInitParams';
export { PermissionInitParams } from './PermissionInitParams';
export type {
  PermissionInitParamsFields,
  PermissionInitParamsJSON,
} from './PermissionInitParams';
export { PermissionSetParams } from './PermissionSetParams';
export type {
  PermissionSetParamsFields,
  PermissionSetParamsJSON,
} from './PermissionSetParams';
export { QueueAddMrEnclaveParams } from './QueueAddMrEnclaveParams';
export type {
  QueueAddMrEnclaveParamsFields,
  QueueAddMrEnclaveParamsJSON,
} from './QueueAddMrEnclaveParams';
export { QueueInitParams } from './QueueInitParams';
export type {
  QueueInitParamsFields,
  QueueInitParamsJSON,
} from './QueueInitParams';
export { QueueRemoveMrEnclaveParams } from './QueueRemoveMrEnclaveParams';
export type {
  QueueRemoveMrEnclaveParamsFields,
  QueueRemoveMrEnclaveParamsJSON,
} from './QueueRemoveMrEnclaveParams';
export { QueueSendRequestParams } from './QueueSendRequestParams';
export type {
  QueueSendRequestParamsFields,
  QueueSendRequestParamsJSON,
} from './QueueSendRequestParams';
export { QuoteInitParams } from './QuoteInitParams';
export type {
  QuoteInitParamsFields,
  QuoteInitParamsJSON,
} from './QuoteInitParams';
export { QuoteInitSimpleParams } from './QuoteInitSimpleParams';
export type {
  QuoteInitSimpleParamsFields,
  QuoteInitSimpleParamsJSON,
} from './QuoteInitSimpleParams';
export { QuoteVerifyParams } from './QuoteVerifyParams';
export type {
  QuoteVerifyParamsFields,
  QuoteVerifyParamsJSON,
} from './QuoteVerifyParams';
export { QuoteVerifySimpleParams } from './QuoteVerifySimpleParams';
export type {
  QuoteVerifySimpleParamsFields,
  QuoteVerifySimpleParamsJSON,
} from './QuoteVerifySimpleParams';
export { VerificationStatus };

export type VerificationStatusKind =
  | VerificationStatus.VerificationPending
  | VerificationStatus.VerificationFailure
  | VerificationStatus.VerificationSuccess
  | VerificationStatus.VerificationOverride;
export type VerificationStatusJSON =
  | VerificationStatus.VerificationPendingJSON
  | VerificationStatus.VerificationFailureJSON
  | VerificationStatus.VerificationSuccessJSON
  | VerificationStatus.VerificationOverrideJSON;

export { SwitchboardPermission };

export type SwitchboardPermissionKind =
  | SwitchboardPermission.PermitNodeheartbeat
  | SwitchboardPermission.PermitQueueUsage;
export type SwitchboardPermissionJSON =
  | SwitchboardPermission.PermitNodeheartbeatJSON
  | SwitchboardPermission.PermitQueueUsageJSON;
