import React, { useState, useEffect } from "react";
import { View } from "views/view";
import { Typography } from "@material-ui/core";
import { httpService } from "common/http-service";

interface IUser {
  about_me: string;
  avatar_id: number;
  id: number;
  login: string;
}

const MyProfile: React.FC = () => {
  const [user, setUser] = useState<IUser>(null);

  useEffect(() => {
    const fetch = async () => {
      const result = await httpService.get({
        url: "user",
        params: {},
      });

      setUser(result.data);
    };
    fetch();
  }, []);

  return (
    <View>
      <Typography variant="h6">My profile</Typography>
      {user ? (
        <>
          <Typography>Login: {user.login}</Typography>
          <Typography>Id: {user.id}</Typography>
          <Typography>About me: {user.about_me}</Typography>
        </>
      ) : (
        <Typography>Data is not loaded!</Typography>
      )}
      <Typography></Typography>
    </View>
  );
};

export { MyProfile };
