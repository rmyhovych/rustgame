#version 450

layout(location = 0) in vec3 a_Pos;
layout(location = 1) in vec3 a_Norm;

layout(location = 0) out vec3 v_out_Color;
layout(location = 1) out vec3 v_out_Norm;
layout(location = 2) out vec3 v_out_FragPos;
layout(location = 3) out vec3 v_out_CamPosition;

layout(set = 0, binding = 0) uniform u_Camera {
    mat4 m_in_PV;
    vec3 v_in_CamPosition;
};

struct ObjectState {
    mat4 m_in_Model;
    vec3 v_in_Color;
    float _padding;
};

layout(set = 0, binding = 1) buffer b_ObjectStates {
    ObjectState s_States[];
};


void main() {
    ObjectState current_state = s_States[gl_InstanceIndex];

    v_out_Color = current_state.v_in_Color;
    v_out_Norm = normalize(transpose(inverse(mat3(current_state.m_in_Model))) * a_Norm);
    v_out_CamPosition = v_in_CamPosition;

    vec4 pos = current_state.m_in_Model * vec4(a_Pos, 1.0);
    v_out_FragPos = vec3(pos);
    gl_Position = m_in_PV * pos;
}
