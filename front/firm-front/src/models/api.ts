import type {
  BaseInfo,
  DeviceHard,
  DeviceSoft,
  Firm,
  InAddFirm,
  InAddHardType,
  InAddSoftType,
  InUpdatePass,
  Login,
  Token,
  Upload,
} from ".";

export class Store {
  static token: string = "";
  static info: BaseInfo | null = null;
  static updateToken(newToken: string) {
    this.token = newToken;
  }

  static baseInfo(info: BaseInfo) {
    this.info = info;
  }
}
const BASE_URL = "http://localhost:3000";

async function post<T>(path: string, data: any): Promise<T> {
  const response = await fetch(`${BASE_URL}${path}`, {
    method: "POST",
    headers: {
      token: Store.token,
      accept: "application/json",
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  });
  if (!response.ok) {
    return Promise.reject(new Error(await response.text()));
  }
  return response.json();
}

async function put<T>(path: string, data: any): Promise<T> {
  const response = await fetch(`${BASE_URL}${path}`, {
    method: "PUT",
    headers: {
      token: Store.token,
      accept: "application/json",
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  });
  if (!response.ok) {
    return Promise.reject(new Error(await response.text()));
  }
  return response.json();
}

async function get<T>(path: string): Promise<T> {
  const response = await fetch(`${BASE_URL}${path}`, {
    method: "GET",
    headers: {
      token: Store.token,
      accept: "application/json",
    },
  });
  if (!response.ok) {
    return Promise.reject(new Error(await response.text()));
  }
  return response.json();
}

async function del<T>(path: string): Promise<T> {
  const response = await fetch(`${BASE_URL}${path}`, {
    method: "DELETE",
    headers: {
      token: Store.token,
      accept: "application/json",
    },
  });
  if (!response.ok) {
    return Promise.reject(new Error(await response.text()));
  }
  return response.json();
}

export class Api {
  static async login(data: Login): Promise<Token> {
    const response = await fetch(`${BASE_URL}/login`, {
      method: "POST",
      body: JSON.stringify(data),
    });
    if (!response.ok) {
      return Promise.reject(new Error(await response.text()));
    }
    const token: Token = await response.json();
    Store.updateToken(token.access_token);
    return Promise.resolve(token);
  }

  static async updatePass(data: InUpdatePass): Promise<Response> {
    return post("/user/updatePass", data);
  }

  static async deviceHard(): Promise<Array<DeviceHard>> {
    return get("/devices");
  }

  static async addeDeviceHard(data: InAddHardType): Promise<Response> {
    return post("/devices", data);
  }

  static async updateDeviceHard(data: DeviceHard): Promise<Response> {
    return put("/devices", data);
  }

  static async baseInfo(): Promise<BaseInfo> {
    return get("/baseInfo");
  }

  static async deviceSoft(): Promise<Array<DeviceSoft>> {
    return get("/softTypes");
  }

  static async addeDeviceSoft(data: InAddSoftType): Promise<Response> {
    return post("/softTypes", data);
  }

  static async updateDeviceSoft(data: DeviceSoft): Promise<Response> {
    return put("/softTypes", data);
  }

  static async firms(): Promise<Array<Firm>> {
    return get("/firms");
  }

  static async updateFirm(data: Firm): Promise<Response> {
    return put("/firms", data);
  }

  static async deleteFirm(data: Firm): Promise<Response> {
    return del(`/firms/${data.id}`);
  }

  static async addFirm(data: InAddFirm): Promise<Response> {
    return post("/firms", data);
  }

  static async deviceFirms(deviceId: number): Promise<Array<Firm>> {
    return get(`/firms/${deviceId}`);
  }

  static async uploadFirm<T>(file: File): Promise<Upload> {
    const form = new FormData();
    form.append("file", file);
    form.append("upload_preset", import.meta.env.VITE_CLOUDINARY_PRESET || "**");
    form.append("api_key", "");
    form.append("source", "ml");
    const response = await fetch(
      "https://api.cloudinary.com/v1_1/xiaolong/upload",
      {
        method: "POST",
        body: form,
      }
    );
    if (!response.ok) {
      return Promise.reject(new Error(await response.text()));
    }
    return response.json();
  }
}
