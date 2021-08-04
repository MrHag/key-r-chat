/*
  TODO: Maybe i shoudl use connect instead of using hook? But for now i have not idea how to implement this
*/

// import { PrivateRoute } from "./PrivateRoute";
// import { connect } from "react-redux";
// import { appStore } from "store";

// const mapStateToProps = (state: appStore) => ({
//   isAuthorized: state.,
//   totalCost: state.basket.totalCost,
// });

// const reduxBasket = connect(mapStateToProps)(PrivateRoute);
// export { reduxBasket as Basket };

export { PrivateRoute } from "./PrivateRoute";
