import { definitionApi } from "@/routes/definitions/$definitionId/-feature/store/api";
import { definitionsApi } from "@/routes/definitions/-feature/store/api";
import { configureStore } from "@reduxjs/toolkit";
import { useDispatch, useSelector } from "react-redux";

export const store = configureStore({
	reducer: {
		[definitionsApi.reducerPath]: definitionsApi.reducer,
		[definitionApi.reducerPath]: definitionApi.reducer,
	},
	middleware: (getDefaultMiddleware) =>
		getDefaultMiddleware().concat(
			definitionsApi.middleware,
			definitionApi.middleware,
		),
});

// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>;
// Inferred type: {posts: PostsState, comments: CommentsState, users: UsersState}
export type AppDispatch = typeof store.dispatch;
export const useAppDispatch = useDispatch.withTypes<AppDispatch>()
export const useAppSelector = useSelector.withTypes<RootState>()