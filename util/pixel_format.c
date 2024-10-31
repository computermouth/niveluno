#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include <stdio.h>

// Function to print SDL pixel format name
const char* getPixelFormatName(Uint32 format) {
    switch (format) {
        case SDL_PIXELFORMAT_UNKNOWN      : return "SDL_PIXELFORMAT_UNKNOWN     ";
        case SDL_PIXELFORMAT_INDEX1LSB    : return "SDL_PIXELFORMAT_INDEX1LSB   ";
        case SDL_PIXELFORMAT_INDEX1MSB    : return "SDL_PIXELFORMAT_INDEX1MSB   ";
        case SDL_PIXELFORMAT_INDEX4LSB    : return "SDL_PIXELFORMAT_INDEX4LSB   ";
        case SDL_PIXELFORMAT_INDEX4MSB    : return "SDL_PIXELFORMAT_INDEX4MSB   ";
        case SDL_PIXELFORMAT_INDEX8       : return "SDL_PIXELFORMAT_INDEX8      ";
        case SDL_PIXELFORMAT_RGB332       : return "SDL_PIXELFORMAT_RGB332      ";
        case SDL_PIXELFORMAT_RGB444       : return "SDL_PIXELFORMAT_RGB444      ";
        case SDL_PIXELFORMAT_BGR444       : return "SDL_PIXELFORMAT_BGR444      ";
        case SDL_PIXELFORMAT_RGB555       : return "SDL_PIXELFORMAT_RGB555      ";
        case SDL_PIXELFORMAT_BGR555       : return "SDL_PIXELFORMAT_BGR555      ";
        case SDL_PIXELFORMAT_ARGB4444     : return "SDL_PIXELFORMAT_ARGB4444    ";
        case SDL_PIXELFORMAT_RGBA4444     : return "SDL_PIXELFORMAT_RGBA4444    ";
        case SDL_PIXELFORMAT_ABGR4444     : return "SDL_PIXELFORMAT_ABGR4444    ";
        case SDL_PIXELFORMAT_BGRA4444     : return "SDL_PIXELFORMAT_BGRA4444    ";
        case SDL_PIXELFORMAT_ARGB1555     : return "SDL_PIXELFORMAT_ARGB1555    ";
        case SDL_PIXELFORMAT_RGBA5551     : return "SDL_PIXELFORMAT_RGBA5551    ";
        case SDL_PIXELFORMAT_ABGR1555     : return "SDL_PIXELFORMAT_ABGR1555    ";
        case SDL_PIXELFORMAT_BGRA5551     : return "SDL_PIXELFORMAT_BGRA5551    ";
        case SDL_PIXELFORMAT_RGB565       : return "SDL_PIXELFORMAT_RGB565      ";
        case SDL_PIXELFORMAT_BGR565       : return "SDL_PIXELFORMAT_BGR565      ";
        case SDL_PIXELFORMAT_RGB24        : return "SDL_PIXELFORMAT_RGB24       ";
        case SDL_PIXELFORMAT_BGR24        : return "SDL_PIXELFORMAT_BGR24       ";
        case SDL_PIXELFORMAT_RGB888       : return "SDL_PIXELFORMAT_RGB888      ";
        case SDL_PIXELFORMAT_RGBX8888     : return "SDL_PIXELFORMAT_RGBX8888    ";
        case SDL_PIXELFORMAT_BGR888       : return "SDL_PIXELFORMAT_BGR888      ";
        case SDL_PIXELFORMAT_BGRX8888     : return "SDL_PIXELFORMAT_BGRX8888    ";
        case SDL_PIXELFORMAT_ARGB8888     : return "SDL_PIXELFORMAT_ARGB8888    ";
        case SDL_PIXELFORMAT_RGBA8888     : return "SDL_PIXELFORMAT_RGBA8888    ";
        case SDL_PIXELFORMAT_ABGR8888     : return "SDL_PIXELFORMAT_ABGR8888    ";
        case SDL_PIXELFORMAT_BGRA8888     : return "SDL_PIXELFORMAT_BGRA8888    ";
        case SDL_PIXELFORMAT_ARGB2101010  : return "SDL_PIXELFORMAT_ARGB2101010 ";
        case SDL_PIXELFORMAT_YV12         : return "SDL_PIXELFORMAT_YV12        ";
        case SDL_PIXELFORMAT_IYUV         : return "SDL_PIXELFORMAT_IYUV        ";
        case SDL_PIXELFORMAT_YUY2         : return "SDL_PIXELFORMAT_YUY2        ";
        case SDL_PIXELFORMAT_UYVY         : return "SDL_PIXELFORMAT_UYVY        ";
        case SDL_PIXELFORMAT_YVYU         : return "SDL_PIXELFORMAT_YVYU        ";
        case SDL_PIXELFORMAT_NV12         : return "SDL_PIXELFORMAT_NV12        ";
        case SDL_PIXELFORMAT_NV21         : return "SDL_PIXELFORMAT_NV21        ";
        case SDL_PIXELFORMAT_EXTERNAL_OES : return "SDL_PIXELFORMAT_EXTERNAL_OES";
        default: return "Unknown Format";
    }
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        printf("Usage: %s <image.png>\n", argv[0]);
        return 1;
    }

    if (SDL_Init(SDL_INIT_VIDEO) != 0) {
        printf("SDL_Init Error: %s\n", SDL_GetError());
        return 1;
    }

    if (IMG_Init(IMG_INIT_PNG) == 0) {
        printf("IMG_Init Error: %s\n", IMG_GetError());
        SDL_Quit();
        return 1;
    }

    SDL_Window* window = SDL_CreateWindow("SDL Pixel Format", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, 640, 480, SDL_WINDOW_SHOWN);
    if (!window) {
        printf("SDL_CreateWindow Error: %s\n", SDL_GetError());
        IMG_Quit();
        SDL_Quit();
        return 1;
    }

    SDL_Renderer* renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC);
    if (!renderer) {
        printf("SDL_CreateRenderer Error: %s\n", SDL_GetError());
        SDL_DestroyWindow(window);
        IMG_Quit();
        SDL_Quit();
        return 1;
    }

    SDL_Surface* surface = IMG_Load(argv[1]);
    if (!surface) {
        printf("IMG_Load Error: %s\n", IMG_GetError());
        SDL_DestroyRenderer(renderer);
        SDL_DestroyWindow(window);
        IMG_Quit();
        SDL_Quit();
        return 1;
    }

    printf("SDL surface format: %s\n", getPixelFormatName(surface->format->format));
    printf("SDL surface pitch: %d\n", surface->pitch);

    SDL_Texture* texture = SDL_CreateTextureFromSurface(renderer, surface);
    if (!texture) {
        printf("SDL_CreateTextureFromSurface Error: %s\n", SDL_GetError());
        SDL_FreeSurface(surface);
        SDL_DestroyRenderer(renderer);
        SDL_DestroyWindow(window);
        IMG_Quit();
        SDL_Quit();
        return 1;
    }

    Uint32 format;
    int access, w, h;
    SDL_QueryTexture(texture, &format, &access, &w, &h);
    printf("SDL texture pixel format: %s\n", getPixelFormatName(format));

    SDL_FreeSurface(surface);
    SDL_DestroyTexture(texture);
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    IMG_Quit();
    SDL_Quit();
    return 0;
}
