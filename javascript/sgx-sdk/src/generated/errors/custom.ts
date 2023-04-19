import { SwitchboardQuoteProgram } from '../../SwitchboardQuoteProgram';
export type CustomError =
  | GenericError
  | InvalidQuoteError
  | QuoteExpiredError
  | InvalidNodeError
  | InsufficientQueueError
  | QueueFullError
  | InvalidSignerError
  | MrEnclaveAlreadyExists
  | MrEnclaveDoesntExist
  | MrEnclaveAtCapacity
  | PermissionDenied;

export class GenericError extends Error {
  static readonly code = 6000;
  readonly code = 6000;
  readonly name = 'GenericError';

  constructor(readonly logs?: string[]) {
    super('6000: ');
  }
}

export class InvalidQuoteError extends Error {
  static readonly code = 6001;
  readonly code = 6001;
  readonly name = 'InvalidQuoteError';

  constructor(readonly logs?: string[]) {
    super('6001: ');
  }
}

export class QuoteExpiredError extends Error {
  static readonly code = 6002;
  readonly code = 6002;
  readonly name = 'QuoteExpiredError';

  constructor(readonly logs?: string[]) {
    super('6002: ');
  }
}

export class InvalidNodeError extends Error {
  static readonly code = 6003;
  readonly code = 6003;
  readonly name = 'InvalidNodeError';

  constructor(readonly logs?: string[]) {
    super('6003: ');
  }
}

export class InsufficientQueueError extends Error {
  static readonly code = 6004;
  readonly code = 6004;
  readonly name = 'InsufficientQueueError';

  constructor(readonly logs?: string[]) {
    super('6004: ');
  }
}

export class QueueFullError extends Error {
  static readonly code = 6005;
  readonly code = 6005;
  readonly name = 'QueueFullError';

  constructor(readonly logs?: string[]) {
    super('6005: ');
  }
}

export class InvalidSignerError extends Error {
  static readonly code = 6006;
  readonly code = 6006;
  readonly name = 'InvalidSignerError';

  constructor(readonly logs?: string[]) {
    super('6006: ');
  }
}

export class MrEnclaveAlreadyExists extends Error {
  static readonly code = 6007;
  readonly code = 6007;
  readonly name = 'MrEnclaveAlreadyExists';

  constructor(readonly logs?: string[]) {
    super('6007: ');
  }
}

export class MrEnclaveDoesntExist extends Error {
  static readonly code = 6008;
  readonly code = 6008;
  readonly name = 'MrEnclaveDoesntExist';

  constructor(readonly logs?: string[]) {
    super('6008: ');
  }
}

export class MrEnclaveAtCapacity extends Error {
  static readonly code = 6009;
  readonly code = 6009;
  readonly name = 'MrEnclaveAtCapacity';

  constructor(readonly logs?: string[]) {
    super('6009: ');
  }
}

export class PermissionDenied extends Error {
  static readonly code = 6010;
  readonly code = 6010;
  readonly name = 'PermissionDenied';

  constructor(readonly logs?: string[]) {
    super('6010: ');
  }
}

export function fromCode(code: number, logs?: string[]): CustomError | null {
  switch (code) {
    case 6000:
      return new GenericError(logs);
    case 6001:
      return new InvalidQuoteError(logs);
    case 6002:
      return new QuoteExpiredError(logs);
    case 6003:
      return new InvalidNodeError(logs);
    case 6004:
      return new InsufficientQueueError(logs);
    case 6005:
      return new QueueFullError(logs);
    case 6006:
      return new InvalidSignerError(logs);
    case 6007:
      return new MrEnclaveAlreadyExists(logs);
    case 6008:
      return new MrEnclaveDoesntExist(logs);
    case 6009:
      return new MrEnclaveAtCapacity(logs);
    case 6010:
      return new PermissionDenied(logs);
  }

  return null;
}
