import subprocess
from datetime import datetime
from datetime import timedelta
from tempfile import NamedTemporaryFile
import math

start = datetime.strptime('2021-01-01', '%Y-%m-%d').date()
end   = datetime.strptime('2022-01-01', '%Y-%m-%d').date()

days = (end - start).days

def daterange(_start, _end):
    for n in range((_end - _start).days):
        yield _start + timedelta(n)

for i in daterange(start, end):
    progress = (i - start).days
    angle = math.floor((progress / days) * 360)

    tmp_distort = NamedTemporaryFile()
    cmd = f'convert icon.png -distort SRT +{angle} {tmp_distort.name}'
    subprocess.run(cmd, shell=True)

    tmp_cropped = NamedTemporaryFile()
    cmd = f'convert {tmp_distort.name} -thumbnail 512x512 -gravity center -extent 512x512 \\( -size 512x512 xc:none -fill white -draw \'circle 256,256 256,0\' \\) -compose CopyOpacity -composite {tmp_cropped.name}'
    subprocess.run(cmd, shell=True)

    cmd = f'convert -font Arial -pointsize 36 -fill black  -stroke white -draw "text 10,30 \'{i}\'" {tmp_cropped.name}  out_{progress:03}.png'
    subprocess.run(cmd, shell=True)
    
    print(angle)
    print (i)
