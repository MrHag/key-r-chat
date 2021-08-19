import axios from "axios";
import { appStore } from "store";

const BASE_ROUTE = "/api/";

interface IPost {
  url: string;
  params?: any;
}

class HttpService {
  async post({ url, params }: IPost) {
    return await axios({
      method: "POST",
      url: BASE_ROUTE + url,
      params,
    });
  }

  async get({ url, params }: IPost) {
    const token = appStore.getState().auth.token;
    console.log("token = ", token);
    return await axios({
      method: "GET",
      url: BASE_ROUTE + url,
      headers: {
        authorization: token,
      },
      params,
    });
  }
}

const httpService = new HttpService();

export { httpService };
