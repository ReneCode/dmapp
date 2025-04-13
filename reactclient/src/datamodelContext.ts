import { createContext } from "react";
import { EDataModel } from "wasm";

export const DatamodelContext = createContext<EDataModel | null>(null);
