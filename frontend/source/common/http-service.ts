import axios from "axios";

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
}

const httpService = new HttpService();

export { httpService };
