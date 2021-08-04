import React from "react";
import { Redirect, Route, RouteProps } from "react-router";
import { SIGN_IN } from "constants/routes";

import { useSelector } from "react-redux";
import { IAppStore } from "store";

const PrivateRoute: React.FC<RouteProps> = ({ ...routeProps }: RouteProps) => {
  const isAuthorized = useSelector(
    (state: IAppStore) => state.user.isAuthorized
  );

  if (isAuthorized) {
    return <Route {...routeProps} />;
  } else {
    return <Redirect to={{ pathname: SIGN_IN }} />;
  }
};

export { PrivateRoute };
