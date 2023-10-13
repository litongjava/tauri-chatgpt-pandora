"""
读取512x512.png 按照下面的文件名和大小输出图片
filename size
128x128.png	128
128x128@2x.png	256
32x32.png	32
icon.icns
icon.ico	16
icon.png	512
Square107x107Logo.png	107
Square142x142Logo.png	142
Square150x150Logo.png	150
Square284x284Logo.png	284
Square30x30Logo.png	30
Square310x310Logo.png	310
Square44x44Logo.png	44
Square71x71Logo.png	71
Square89x89Logo.png	89
StoreLogo.png	50
"""
from PIL import Image

# 读取原始图像
srcImage = r"F:\code\rust\project-litongjava\tauri-chatgpt-work\icons\512x512.png"
image = Image.open(srcImage)

# 输出不同尺寸的图像
image.resize((128, 128)).save("128x128.png")
image.resize((256, 256)).save("128x128@2x.png")
image.resize((32, 32)).save("32x32.png")
image.resize((512, 512)).save("icon.png")
image.resize((107, 107)).save("Square107x107Logo.png")
image.resize((142, 142)).save("Square142x142Logo.png")
image.resize((150, 150)).save("Square150x150Logo.png")
image.resize((284, 284)).save("Square284x284Logo.png")
image.resize((30, 30)).save("Square30x30Logo.png")
image.resize((310, 310)).save("Square310x310Logo.png")
image.resize((44, 44)).save("Square44x44Logo.png")
image.resize((71, 71)).save("Square71x71Logo.png")
image.resize((89, 89)).save("Square89x89Logo.png")
image.resize((50, 50)).save("StoreLogo.png")
Image.open(srcImage).resize((16, 16)).save("icon.ico")
Image.open(srcImage).resize((1024, 1024)).save("icon.icns")
