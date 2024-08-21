import {useEffect, useState} from "react";
import IPageModel from "../../../types/page/IPageModel";

export const usePageRemoveModal = () => {
  const [isOpen, updateOpen] = useState<boolean>(false);
  const [valToRemove, setValToRemove] = useState<IPageModel | null>(null)

  useEffect(() => {
    if (!isOpen) {
      setTimeout(() => {
        setValToRemove(null)
      }, 1000)
    }
  }, [isOpen]);

  return { isOpen, updateOpen, setValToRemove, valToRemove }
}
