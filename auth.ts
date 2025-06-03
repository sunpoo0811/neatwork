import { defineStore } from 'pinia'
import { ref } from 'vue'
import CasdoorUser from 'casdoor-vue-sdk'
import Permission from 'casdoor-vue-sdk'
import Role from 'casdoor-vue-sdk'
import { useCasdoor } from 'casdoor-vue-sdk'



interface AuthState {
    user: CasdoorUser | null
    roles: Role[]
    permissions: Permission[]
    isInitialized: boolean
}

export const useAuthStore = defineStore('auth', () => {
    const state = ref<AuthState>({
        user: null,
        roles: [],
        permissions: [],
        isInitialized: false
    })

    // 初始化认证状态
    const initialize = async () => {
        const { getUser, getRoles, getPermissions } = useCasdoor()

        try {
            const [user, roles, permissions] = await Promise.all([
                getUser(),
                getRoles(),
                getPermissions()
            ])

            state.value = {
                user,
                roles,
                permissions,
                isInitialized: true
            }
        } catch (error) {
            console.error('Auth initialization failed:', error)
            reset()
        }
    }

    // 重置状态
    const reset = () => {
        state.value = {
            user: null,
            roles: [],
            permissions: [],
            isInitialized: false
        }
    }

    // 检查权限
    const hasPermission = (permission: string) => {
        return state.value.permissions.some(p => p.name === permission)
    }

    // 检查角色
    const hasRole = (role: string) => {
        return state.value.roles.some(r => r.name === role)
    }

    return {
        state,
        initialize,
        reset,
        hasPermission,
        hasRole
    }
})