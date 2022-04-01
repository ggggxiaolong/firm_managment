export type HardCategory = "Lock" | "Box";
export interface DeviceHard {
  readonly id: number;
  readonly hard_version: string;
  readonly name: string;
  readonly category: HardCategory;
  readonly has_ble: boolean;
  readonly has_finger: boolean;
  readonly has_stm32: boolean;
  readonly desc: string;
}

export interface DeviceSoft {
  readonly id: number;
  readonly name: string;
}

export interface BaseInfo {
  readonly hard: Array<DeviceHard>;
  readonly soft: Array<DeviceSoft>;
}

export interface User {
  readonly name: string;
  readonly ticker: number;
}

export interface Token {
  readonly access_token: string;
  readonly user: User;
}

export interface InAddFirm {
  readonly hard_version: number;
  readonly version_name: string;
  readonly version_format: string;
  readonly version_type: number;
  readonly finger_level: number;
  readonly url: string;
  readonly desc: string;
  readonly update_time: number;
  readonly rely_version_type?: number;
  readonly min?: string;
  readonly max?: string;
  readonly des_en: string;
  readonly des_ko: string;
  readonly des_sp: string;
}

export interface InAddHardType {
  readonly hard_version: string;
  readonly name: string;
  readonly category: HardCategory;
  readonly has_ble: boolean;
  readonly has_finger: boolean;
  readonly has_stm32: boolean;
  readonly desc: string;
}

export interface InAddSoftType {
  readonly name: string;
}

export interface Firm {
  readonly id: number;
  readonly hard_version: number;
  readonly version_name: string;
  readonly version_format: string;
  readonly version_type: number;
  readonly finger_level: number;
  readonly url: string;
  readonly desc: string;
  readonly update_time: number;
  readonly rely_version_type?: number;
  readonly min?: string;
  readonly max?: string;
  readonly des_en: string;
  readonly des_ko: string;
  readonly des_sp: string;
}

export interface Login {
  readonly email: string;
  readonly password: string;
}

export interface InUpdatePass {
  readonly old_pass: string;
  readonly new_pass: string;
}

export interface Upload {
  readonly public_id: string;
  readonly secure_url: string;
}

export interface ApiResponse {
  readonly message: string;
}

export class ApiError {
  status: number;
  message: string;
  constructor(status: number, message: string){
    this.status = status;
    this.message = message;
  }
}
