/*
struct VertexOutput
{
    float4 position : SV_POSITION;
    float2 texCoord: TEXCOORD0;
    float4 color: COLOR;
};

struct Vertex
{
    float2 pos; float2 texCoord;
    float4 color;
};

struct Constants
{
    float2 scale;
    float2 translation;
};

StructuredBuffer<Vertex> g_vertexData : register(t0, space0);
ConstantBuffer<Constants> g_constants : register(b1, space0);

VertexOutput main(uint vertexIndex : SV_VertexID)
{
    Vertex vertex = g_vertexData[vertexIndex];

    VertexOutput o;
    o.position = float4(vertex.pos * g_constants.scale + g_constants.translation, 0.0, 1.0);
    o.texCoord = vertex.texCoord;
    o.color = vertex.color;
    return o;
}
*/
pub(crate) const IMGUI_VS: &[u8] = &[
    3, 2, 35, 7, 0, 0, 1, 0, 0, 0, 14, 0, 46, 0, 0, 0, 0, 0, 0, 0, 17, 0, 2, 0, 1, 0, 0, 0, 14, 0,
    3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 15, 0, 9, 0, 0, 0, 0, 0, 1, 0, 0, 0, 109, 97, 105, 110, 0, 0, 0,
    0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 3, 0, 3, 0, 5, 0, 0, 0, 118, 2, 0, 0, 5, 0,
    10, 0, 6, 0, 0, 0, 116, 121, 112, 101, 46, 83, 116, 114, 117, 99, 116, 117, 114, 101, 100, 66,
    117, 102, 102, 101, 114, 46, 86, 101, 114, 116, 101, 120, 0, 0, 0, 0, 5, 0, 4, 0, 7, 0, 0, 0,
    86, 101, 114, 116, 101, 120, 0, 0, 6, 0, 4, 0, 7, 0, 0, 0, 0, 0, 0, 0, 112, 111, 115, 0, 6, 0,
    6, 0, 7, 0, 0, 0, 1, 0, 0, 0, 116, 101, 120, 67, 111, 111, 114, 100, 0, 0, 0, 0, 6, 0, 5, 0, 7,
    0, 0, 0, 2, 0, 0, 0, 99, 111, 108, 111, 114, 0, 0, 0, 5, 0, 6, 0, 8, 0, 0, 0, 103, 95, 118,
    101, 114, 116, 101, 120, 68, 97, 116, 97, 0, 0, 0, 0, 5, 0, 10, 0, 9, 0, 0, 0, 116, 121, 112,
    101, 46, 67, 111, 110, 115, 116, 97, 110, 116, 66, 117, 102, 102, 101, 114, 46, 67, 111, 110,
    115, 116, 97, 110, 116, 115, 0, 0, 0, 6, 0, 5, 0, 9, 0, 0, 0, 0, 0, 0, 0, 115, 99, 97, 108,
    101, 0, 0, 0, 6, 0, 6, 0, 9, 0, 0, 0, 1, 0, 0, 0, 116, 114, 97, 110, 115, 108, 97, 116, 105,
    111, 110, 0, 5, 0, 5, 0, 10, 0, 0, 0, 103, 95, 99, 111, 110, 115, 116, 97, 110, 116, 115, 0, 5,
    0, 7, 0, 4, 0, 0, 0, 111, 117, 116, 46, 118, 97, 114, 46, 84, 69, 88, 67, 79, 79, 82, 68, 48,
    0, 0, 0, 5, 0, 6, 0, 5, 0, 0, 0, 111, 117, 116, 46, 118, 97, 114, 46, 67, 79, 76, 79, 82, 0, 0,
    0, 5, 0, 4, 0, 1, 0, 0, 0, 109, 97, 105, 110, 0, 0, 0, 0, 71, 0, 4, 0, 2, 0, 0, 0, 11, 0, 0, 0,
    42, 0, 0, 0, 71, 0, 4, 0, 3, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 71, 0, 4, 0, 4, 0, 0, 0, 30, 0,
    0, 0, 0, 0, 0, 0, 71, 0, 4, 0, 5, 0, 0, 0, 30, 0, 0, 0, 1, 0, 0, 0, 71, 0, 4, 0, 8, 0, 0, 0,
    34, 0, 0, 0, 0, 0, 0, 0, 71, 0, 4, 0, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 71, 0, 4, 0, 10, 0,
    0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 71, 0, 4, 0, 10, 0, 0, 0, 33, 0, 0, 0, 1, 0, 0, 0, 72, 0, 5, 0,
    7, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 0, 0, 0, 72, 0, 5, 0, 7, 0, 0, 0, 1, 0, 0, 0, 35, 0, 0,
    0, 8, 0, 0, 0, 72, 0, 5, 0, 7, 0, 0, 0, 2, 0, 0, 0, 35, 0, 0, 0, 16, 0, 0, 0, 71, 0, 4, 0, 11,
    0, 0, 0, 6, 0, 0, 0, 32, 0, 0, 0, 72, 0, 5, 0, 6, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 0, 0, 0,
    72, 0, 4, 0, 6, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 71, 0, 3, 0, 6, 0, 0, 0, 3, 0, 0, 0, 72, 0,
    5, 0, 9, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 0, 0, 0, 72, 0, 5, 0, 9, 0, 0, 0, 1, 0, 0, 0, 35,
    0, 0, 0, 8, 0, 0, 0, 71, 0, 3, 0, 9, 0, 0, 0, 2, 0, 0, 0, 21, 0, 4, 0, 12, 0, 0, 0, 32, 0, 0,
    0, 1, 0, 0, 0, 43, 0, 4, 0, 12, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 43, 0, 4, 0, 12, 0, 0, 0, 14,
    0, 0, 0, 1, 0, 0, 0, 22, 0, 3, 0, 15, 0, 0, 0, 32, 0, 0, 0, 43, 0, 4, 0, 15, 0, 0, 0, 16, 0, 0,
    0, 0, 0, 0, 0, 43, 0, 4, 0, 15, 0, 0, 0, 17, 0, 0, 0, 0, 0, 128, 63, 23, 0, 4, 0, 18, 0, 0, 0,
    15, 0, 0, 0, 2, 0, 0, 0, 23, 0, 4, 0, 19, 0, 0, 0, 15, 0, 0, 0, 4, 0, 0, 0, 30, 0, 5, 0, 7, 0,
    0, 0, 18, 0, 0, 0, 18, 0, 0, 0, 19, 0, 0, 0, 29, 0, 3, 0, 11, 0, 0, 0, 7, 0, 0, 0, 30, 0, 3, 0,
    6, 0, 0, 0, 11, 0, 0, 0, 32, 0, 4, 0, 20, 0, 0, 0, 2, 0, 0, 0, 6, 0, 0, 0, 30, 0, 4, 0, 9, 0,
    0, 0, 18, 0, 0, 0, 18, 0, 0, 0, 32, 0, 4, 0, 21, 0, 0, 0, 2, 0, 0, 0, 9, 0, 0, 0, 21, 0, 4, 0,
    22, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 32, 0, 4, 0, 23, 0, 0, 0, 1, 0, 0, 0, 22, 0, 0, 0, 32, 0,
    4, 0, 24, 0, 0, 0, 3, 0, 0, 0, 19, 0, 0, 0, 32, 0, 4, 0, 25, 0, 0, 0, 3, 0, 0, 0, 18, 0, 0, 0,
    19, 0, 2, 0, 26, 0, 0, 0, 33, 0, 3, 0, 27, 0, 0, 0, 26, 0, 0, 0, 32, 0, 4, 0, 28, 0, 0, 0, 2,
    0, 0, 0, 7, 0, 0, 0, 32, 0, 4, 0, 29, 0, 0, 0, 2, 0, 0, 0, 18, 0, 0, 0, 59, 0, 4, 0, 20, 0, 0,
    0, 8, 0, 0, 0, 2, 0, 0, 0, 59, 0, 4, 0, 21, 0, 0, 0, 10, 0, 0, 0, 2, 0, 0, 0, 59, 0, 4, 0, 23,
    0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 59, 0, 4, 0, 24, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 59, 0, 4, 0,
    25, 0, 0, 0, 4, 0, 0, 0, 3, 0, 0, 0, 59, 0, 4, 0, 24, 0, 0, 0, 5, 0, 0, 0, 3, 0, 0, 0, 54, 0,
    5, 0, 26, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 248, 0, 2, 0, 30, 0, 0, 0, 61, 0, 4, 0,
    22, 0, 0, 0, 31, 0, 0, 0, 2, 0, 0, 0, 65, 0, 6, 0, 28, 0, 0, 0, 32, 0, 0, 0, 8, 0, 0, 0, 13, 0,
    0, 0, 31, 0, 0, 0, 61, 0, 4, 0, 7, 0, 0, 0, 33, 0, 0, 0, 32, 0, 0, 0, 81, 0, 5, 0, 18, 0, 0, 0,
    34, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 81, 0, 5, 0, 18, 0, 0, 0, 35, 0, 0, 0, 33, 0, 0, 0, 1, 0,
    0, 0, 81, 0, 5, 0, 19, 0, 0, 0, 36, 0, 0, 0, 33, 0, 0, 0, 2, 0, 0, 0, 65, 0, 5, 0, 29, 0, 0, 0,
    37, 0, 0, 0, 10, 0, 0, 0, 13, 0, 0, 0, 61, 0, 4, 0, 18, 0, 0, 0, 38, 0, 0, 0, 37, 0, 0, 0, 133,
    0, 5, 0, 18, 0, 0, 0, 39, 0, 0, 0, 34, 0, 0, 0, 38, 0, 0, 0, 65, 0, 5, 0, 29, 0, 0, 0, 40, 0,
    0, 0, 10, 0, 0, 0, 14, 0, 0, 0, 61, 0, 4, 0, 18, 0, 0, 0, 41, 0, 0, 0, 40, 0, 0, 0, 129, 0, 5,
    0, 18, 0, 0, 0, 42, 0, 0, 0, 39, 0, 0, 0, 41, 0, 0, 0, 81, 0, 5, 0, 15, 0, 0, 0, 43, 0, 0, 0,
    42, 0, 0, 0, 0, 0, 0, 0, 81, 0, 5, 0, 15, 0, 0, 0, 44, 0, 0, 0, 42, 0, 0, 0, 1, 0, 0, 0, 80, 0,
    7, 0, 19, 0, 0, 0, 45, 0, 0, 0, 43, 0, 0, 0, 44, 0, 0, 0, 16, 0, 0, 0, 17, 0, 0, 0, 62, 0, 3,
    0, 3, 0, 0, 0, 45, 0, 0, 0, 62, 0, 3, 0, 4, 0, 0, 0, 35, 0, 0, 0, 62, 0, 3, 0, 5, 0, 0, 0, 36,
    0, 0, 0, 253, 0, 1, 0, 56, 0, 1, 0,
];

/*
SamplerState g_sampler : register(s2, space0);
Texture2D g_texture : register(t3, space0);

struct VertexInput
{
    float4 position : SV_POSITION;
    float2 texCoord: TEXCOORD0;
    float4 color: COLOR;
};

float4 main(VertexInput input) : SV_Target0
{
    return input.color * g_texture.Sample(g_sampler, input.texCoord);
}
*/
pub(crate) const IMGUI_PS: &[u8] = &[
    3, 2, 35, 7, 0, 0, 1, 0, 0, 0, 14, 0, 29, 0, 0, 0, 0, 0, 0, 0, 17, 0, 2, 0, 1, 0, 0, 0, 14, 0,
    3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 15, 0, 9, 0, 4, 0, 0, 0, 1, 0, 0, 0, 109, 97, 105, 110, 0, 0, 0,
    0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 16, 0, 3, 0, 1, 0, 0, 0, 7, 0, 0, 0, 3, 0,
    3, 0, 5, 0, 0, 0, 118, 2, 0, 0, 5, 0, 6, 0, 6, 0, 0, 0, 116, 121, 112, 101, 46, 115, 97, 109,
    112, 108, 101, 114, 0, 0, 0, 0, 5, 0, 5, 0, 7, 0, 0, 0, 103, 95, 115, 97, 109, 112, 108, 101,
    114, 0, 0, 0, 5, 0, 6, 0, 8, 0, 0, 0, 116, 121, 112, 101, 46, 50, 100, 46, 105, 109, 97, 103,
    101, 0, 0, 0, 5, 0, 5, 0, 9, 0, 0, 0, 103, 95, 116, 101, 120, 116, 117, 114, 101, 0, 0, 0, 5,
    0, 7, 0, 3, 0, 0, 0, 105, 110, 46, 118, 97, 114, 46, 84, 69, 88, 67, 79, 79, 82, 68, 48, 0, 0,
    0, 0, 5, 0, 6, 0, 4, 0, 0, 0, 105, 110, 46, 118, 97, 114, 46, 67, 79, 76, 79, 82, 0, 0, 0, 0,
    5, 0, 7, 0, 5, 0, 0, 0, 111, 117, 116, 46, 118, 97, 114, 46, 83, 86, 95, 84, 97, 114, 103, 101,
    116, 48, 0, 0, 5, 0, 4, 0, 1, 0, 0, 0, 109, 97, 105, 110, 0, 0, 0, 0, 5, 0, 7, 0, 10, 0, 0, 0,
    116, 121, 112, 101, 46, 115, 97, 109, 112, 108, 101, 100, 46, 105, 109, 97, 103, 101, 0, 0, 71,
    0, 4, 0, 2, 0, 0, 0, 11, 0, 0, 0, 15, 0, 0, 0, 71, 0, 4, 0, 3, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0,
    0, 71, 0, 4, 0, 4, 0, 0, 0, 30, 0, 0, 0, 1, 0, 0, 0, 71, 0, 4, 0, 5, 0, 0, 0, 30, 0, 0, 0, 0,
    0, 0, 0, 71, 0, 4, 0, 7, 0, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 71, 0, 4, 0, 7, 0, 0, 0, 33, 0, 0,
    0, 2, 0, 0, 0, 71, 0, 4, 0, 9, 0, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 71, 0, 4, 0, 9, 0, 0, 0, 33,
    0, 0, 0, 3, 0, 0, 0, 26, 0, 2, 0, 6, 0, 0, 0, 32, 0, 4, 0, 11, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0,
    22, 0, 3, 0, 12, 0, 0, 0, 32, 0, 0, 0, 25, 0, 9, 0, 8, 0, 0, 0, 12, 0, 0, 0, 1, 0, 0, 0, 2, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 32, 0, 4, 0, 13, 0, 0, 0, 0, 0, 0, 0, 8,
    0, 0, 0, 23, 0, 4, 0, 14, 0, 0, 0, 12, 0, 0, 0, 4, 0, 0, 0, 32, 0, 4, 0, 15, 0, 0, 0, 1, 0, 0,
    0, 14, 0, 0, 0, 23, 0, 4, 0, 16, 0, 0, 0, 12, 0, 0, 0, 2, 0, 0, 0, 32, 0, 4, 0, 17, 0, 0, 0, 1,
    0, 0, 0, 16, 0, 0, 0, 32, 0, 4, 0, 18, 0, 0, 0, 3, 0, 0, 0, 14, 0, 0, 0, 19, 0, 2, 0, 19, 0, 0,
    0, 33, 0, 3, 0, 20, 0, 0, 0, 19, 0, 0, 0, 27, 0, 3, 0, 10, 0, 0, 0, 8, 0, 0, 0, 59, 0, 4, 0,
    11, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 59, 0, 4, 0, 13, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 59, 0,
    4, 0, 15, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 59, 0, 4, 0, 17, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0,
    59, 0, 4, 0, 15, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 59, 0, 4, 0, 18, 0, 0, 0, 5, 0, 0, 0, 3, 0,
    0, 0, 54, 0, 5, 0, 19, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 248, 0, 2, 0, 21, 0, 0, 0,
    61, 0, 4, 0, 16, 0, 0, 0, 22, 0, 0, 0, 3, 0, 0, 0, 61, 0, 4, 0, 14, 0, 0, 0, 23, 0, 0, 0, 4, 0,
    0, 0, 61, 0, 4, 0, 8, 0, 0, 0, 24, 0, 0, 0, 9, 0, 0, 0, 61, 0, 4, 0, 6, 0, 0, 0, 25, 0, 0, 0,
    7, 0, 0, 0, 86, 0, 5, 0, 10, 0, 0, 0, 26, 0, 0, 0, 24, 0, 0, 0, 25, 0, 0, 0, 87, 0, 6, 0, 14,
    0, 0, 0, 27, 0, 0, 0, 26, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 133, 0, 5, 0, 14, 0, 0, 0, 28, 0,
    0, 0, 23, 0, 0, 0, 27, 0, 0, 0, 62, 0, 3, 0, 5, 0, 0, 0, 28, 0, 0, 0, 253, 0, 1, 0, 56, 0, 1,
    0,
];
