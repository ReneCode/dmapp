import { createContext } from "react";
import { ECAPI } from "wasm";

export const APIContext = createContext<ECAPI>(null as any);
