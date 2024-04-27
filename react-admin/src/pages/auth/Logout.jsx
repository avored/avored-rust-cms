import {useEffect} from "react"
import logo from "../../assets/logo_only.svg"
import { useNavigate } from "react-router-dom"
import { isEmpty } from "lodash"
import InputField from "../../components/InputField"
import {useForm} from 'react-hook-form'
import {joiResolver} from '@hookform/resolvers/joi'
import { useLogin } from "./hooks/useLogin"
import { loginSchema } from "./schemas/login.schema"
import _ from 'lodash'
import { ErrorMessage } from "../../components/ErrorMessage"

function Logout() {
  const redirect = useNavigate();
  useEffect(() => {
    localStorage.clear()
    redirect("/admin/login")
  }, []);

}

export default Logout;
