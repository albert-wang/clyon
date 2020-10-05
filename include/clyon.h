#include <stdint.h>

extern "C"
{
	struct LyonPathBuilder;
	struct LyonPath;
	struct LyonGeometry16;
	struct LyonGeometry32;
	
	LyonPathBuilder* CreatePathBuilder(uint32_t attrs);

	struct LyonInputVertex
	{
		float position[2];
		float uv[2];
		float color[4];
		float custom[3];
	};

	inline LyonInputVertex point(float x, float y)
	{
		LyonInputVertex v = { 0 };
		v.position[0] = x;
		v.position[1] = y;

		return v;
	}

	struct LyonOutputVertex
	{
		float position[2];
		float uv[2];
		float normal[2];
		uint32_t color;
		float custom[3];
	};

	void PathBuilder_MoveTo(LyonPathBuilder*, LyonInputVertex);
	void PathBuilder_LineTo(LyonPathBuilder*, LyonInputVertex);
	void PathBuilder_QuadraticBeizerTo(LyonPathBuilder*, float ctrlX, float ctrlY, LyonInputVertex);
	void PathBuilder_CubicBeizerTo(LyonPathBuilder*, float ctrlX, float ctrlY, float ctrl2X, float ctrl2Y, LyonInputVertex);

	LyonPath* PathBuilder_Build(LyonPathBuilder*);

	LyonGeometry16* TessellateFill16(LyonPath*);
	LyonGeometry16* TessellateStroke16(LyonPath*);

	uint32_t Geometry_VerticesLength16(LyonGeometry16*);
	uint32_t Geometry_IndicesLength16(LyonGeometry16*);

	const LyonOutputVertex* Geometry_VerticesData16(LyonGeometry16*);
	const uint16_t* Geometry_IndicesData16(LyonGeometry16*);

	void FreeGeometry16(LyonGeometry16*);

	LyonGeometry32* TessellateFill32(LyonPath*);
	LyonGeometry32* TessellateStroke32(LyonPath*);

	uint32_t Geometry_VerticesLength32(LyonGeometry32*);
	uint32_t Geometry_IndicesLength32(LyonGeometry32*);

	const LyonOutputVertex* Geometry_VerticesData32(LyonGeometry32*);
	const uint16_t* Geometry_IndicesData32(LyonGeometry32*);

	void FreeGeometry32(LyonGeometry32*);
}